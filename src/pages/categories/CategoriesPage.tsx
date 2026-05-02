import { useState } from "react";
import {
  Stack,
  Group,
  ActionIcon,
  Tooltip,
  Text,
  Modal,
  Badge,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import { IconPlus, IconEdit, IconTrash } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { SearchInput } from "../../components/common/SearchInput";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { CategoryForm } from "../../components/forms/CategoryForm";
import { categoriesApi, type Category } from "../../api/categories";
import { useDebounce } from "../../hooks/useDebounce";
import { useAuthStore } from "../../store/authStore";

const PAGE_SIZE = 20;

export function CategoriesPage() {
  const queryClient = useQueryClient();
  const { user } = useAuthStore();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [editing, setEditing] = useState<Category | null>(null);
  const [opened, { open, close }] = useDisclosure(false);
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["categories", page, debouncedSearch],
    queryFn: () =>
      categoriesApi.list({
        page: Number(page),
        per_page: Number(PAGE_SIZE),
        company_id: user?.company_id,
        ...(debouncedSearch?.trim() && { search: debouncedSearch }),
      }),
  });

  const createMutation = useMutation({
    mutationFn: (v: any) =>
      categoriesApi.create({ 
        ...v, 
        company_id: user?.company_id,
        code: v.code || v.name?.toUpperCase().replace(/\s+/g, '-').slice(0, 10) || 'DEFAULT',
        sort_order: v.sort_order || 0
      }),
    onSuccess: () => {
      notifications.show({
        title: "Created",
        message: "Category created.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["categories"] });
      close();
    },
    onError: (error: any) => {
      const message = error?.response?.data?.message || "Failed to create category";
      notifications.show({
        title: "Error",
        message,
        color: "red",
      });
    },
  });

  const updateMutation = useMutation({
    mutationFn: (v: any) => categoriesApi.update(editing!.id, v),
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "Category updated.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["categories"] });
      close();
      setEditing(null);
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => categoriesApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Category deleted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["categories"] });
    },
  });

  const handleEdit = (cat: Category) => {
    setEditing(cat);
    open();
  };

  return (
    <Stack>
      <PageHeader
        title="Categories"
        description="Manage inventory categories"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Categories" }]}
        action={{
          label: "Add Category",
          icon: <IconPlus size={16} />,
          onClick: () => {
            setEditing(null);
            open();
          },
        }}
      />
      <SearchInput value={search} onChange={setSearch} w={280} />
      <DataTable
        records={data?.data || []}
        fetching={isLoading}
        totalRecords={data?.total_items || 0}
        recordsPerPage={PAGE_SIZE}
        page={page}
        onPageChange={setPage}
        columns={[
          { accessor: "name", title: "Category Name" },
          {
            accessor: "parent_name",
            title: "Parent",
            render: (c) => c.parent_name || "—",
          },
          {
            accessor: "description",
            title: "Description",
            render: (c) => c.description || "—",
          },
          {
            accessor: "item_count",
            title: "Items",
            render: (c) => <Badge variant="light">{c.item_count || 0}</Badge>,
          },
          {
            accessor: "actions",
            title: "",
            width: 90,
            render: (cat: Category) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="Edit">
                  <ActionIcon
                    variant="subtle"
                    color="blue"
                    onClick={() => handleEdit(cat)}
                  >
                    <IconEdit size={16} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label="Delete">
                  <ActionIcon
                    variant="subtle"
                    color="red"
                    onClick={() =>
                      openConfirmModal({
                        title: "Delete Category",
                        message: `Delete "${cat.name}"?`,
                        danger: true,
                        onConfirm: () => deleteMutation.mutate(cat.id),
                      })
                    }
                  >
                    <IconTrash size={16} />
                  </ActionIcon>
                </Tooltip>
              </Group>
            ),
          },
        ]}
        highlightOnHover
        withTableBorder
        borderRadius="md"
        striped
      />

      <Modal
        opened={opened}
        onClose={() => {
          close();
          setEditing(null);
        }}
        title={editing ? "Edit Category" : "Create Category"}
        size="md"
      >
        <CategoryForm
          initialValues={editing}
          onSubmit={async (v) => {
            if (editing) await updateMutation.mutateAsync(v);
            else await createMutation.mutateAsync(v);
          }}
          loading={createMutation.isPending || updateMutation.isPending}
        />
      </Modal>
    </Stack>
  );
}
