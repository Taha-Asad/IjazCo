import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Stack, Group, ActionIcon, Tooltip, Modal, Badge } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import { IconPlus, IconEye, IconTrash } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { SearchInput } from "../../components/common/SearchInput";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { SupplierForm } from "../../components/forms/SupplierForm";
import { suppliersApi, type Supplier } from "../../api/suppliers";
import { useDebounce } from "../../hooks/useDebounce";
import { useAuthStore } from "../../store/authStore";
import { formatCurrency } from "../../utils/formatters";

const PAGE_SIZE = 20;

export function SuppliersPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { user } = useAuthStore();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [opened, { open, close }] = useDisclosure(false);
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["suppliers", page, debouncedSearch],
    queryFn: () =>
      suppliersApi.list({
        page: Number(page),
        per_page: Number(PAGE_SIZE),
        ...(debouncedSearch?.trim() && { search: debouncedSearch }),
      }),
  });

  const createMutation = useMutation({
    mutationFn: (v: any) =>
      suppliersApi.create({ ...v, company_id: user?.company_id }),
    onSuccess: () => {
      notifications.show({
        title: "Created",
        message: "Supplier created.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["suppliers"] });
      close();
    },
    onError: (error: any) => {
      const message = error?.response?.data?.message || "Failed to create supplier";
      notifications.show({
        title: "Error",
        message,
        color: "red",
      });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => suppliersApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Supplier deleted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["suppliers"] });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Suppliers"
        description="Manage supplier accounts"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Suppliers" }]}
        action={{
          label: "Add Supplier",
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
          { accessor: "name", title: "Supplier Name" },
          { accessor: "contact_person", title: "Contact" },
          { accessor: "email", title: "Email" },
          { accessor: "country", title: "Country" },
          {
            accessor: "payment_terms",
            title: "Payment Terms",
            render: (s) => (s.payment_terms ? `${s.payment_terms} days` : "—"),
          },
          {
            accessor: "total_spent",
            title: "Total Orders",
            render: (s) => formatCurrency(s.total_spent || 0),
          },
          {
            accessor: "actions",
            title: "",
            width: 80,
            render: (s: Supplier) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="View">
                  <ActionIcon
                    variant="subtle"
                    onClick={() => navigate(`/suppliers/${s.id}`)}
                  >
                    <IconEye size={16} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label="Delete">
                  <ActionIcon
                    variant="subtle"
                    color="red"
                    onClick={() =>
                      openConfirmModal({
                        title: "Delete Supplier",
                        message: `Delete supplier "${s.name}"?`,
                        danger: true,
                        onConfirm: () => deleteMutation.mutate(s.id),
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
      <Modal opened={opened} onClose={close} title="Add Supplier" size="md">
        <SupplierForm
          onSubmit={async (v) => {
            await createMutation.mutateAsync(v);
          }}
          loading={createMutation.isPending}
        />
      </Modal>
    </Stack>
  );
}
