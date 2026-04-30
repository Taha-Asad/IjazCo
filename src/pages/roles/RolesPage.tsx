import { useState } from "react";
import {
  Stack,
  Group,
  Badge,
  ActionIcon,
  Tooltip,
  Modal,
  Text,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import { IconPlus, IconEdit, IconTrash } from "@tabler/icons-react";
import { useNavigate } from "react-router-dom";
import { PageHeader } from "../../components/common/PageHeader";
import { SearchInput } from "../../components/common/SearchInput";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { RoleForm } from "../../components/forms/RoleForm";
import { rolesApi } from "../../api/roles";
import { useDebounce } from "../../hooks/useDebounce";
import { useAuthStore } from "../../store/authStore";

const PAGE_SIZE = 20;

export function RolesPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { user } = useAuthStore();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [opened, { open, close }] = useDisclosure(false);
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["roles", page, debouncedSearch],
    queryFn: () =>
      rolesApi.list({ page, per_page: PAGE_SIZE, search: debouncedSearch }),
  });

  const createMutation = useMutation({
    mutationFn: (values: any) =>
      rolesApi.create({ ...values, company_id: user?.company_id }),
    onSuccess: () => {
      notifications.show({
        title: "Created",
        message: "Role created.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["roles"] });
      close();
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => rolesApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Role deleted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["roles"] });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Roles"
        description="Manage user roles and permissions"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Roles" }]}
        action={{
          label: "Create Role",
          icon: <IconPlus size={16} />,
          onClick: open,
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
          { accessor: "name", title: "Role Name" },
          { accessor: "description", title: "Description" },
          {
            accessor: "user_count",
            title: "Users",
            render: (r) => <Badge variant="light">{r.user_count || 0}</Badge>,
          },
          {
            accessor: "actions",
            title: "",
            width: 90,
            render: (r) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="Edit">
                  <ActionIcon
                    variant="subtle"
                    color="blue"
                    onClick={() => navigate(`/roles/${r.id}`)}
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
                        title: "Delete Role",
                        message: `Delete role "${r.name}"?`,
                        danger: true,
                        onConfirm: () => deleteMutation.mutate(r.id),
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

      <Modal opened={opened} onClose={close} title="Create Role" size="lg">
        <RoleForm
          onSubmit={async (v) => {
            await createMutation.mutateAsync(v);
          }}
          loading={createMutation.isPending}
        />
      </Modal>
    </Stack>
  );
}
