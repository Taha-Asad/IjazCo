import { useState } from "react";
import { useNavigate } from "react-router-dom";
import {
  Stack,
  Group,
  ActionIcon,
  Tooltip,
  Badge,
  Modal,
  Text,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import { IconPlus, IconEye, IconTrash } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { SearchInput } from "../../components/common/SearchInput";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { CustomerForm } from "../../components/forms/CustomerForm";
import { customersApi, type Customer } from "../../api/customers";
import { useDebounce } from "../../hooks/useDebounce";
import { formatCurrency } from "../../utils/formatters";
import { useAuthStore } from "../../store/authStore";

const PAGE_SIZE = 20;

export function CustomersPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { user } = useAuthStore();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [opened, { open, close }] = useDisclosure(false);
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["customers", page, debouncedSearch],
    queryFn: () =>
      customersApi.list({
        page: Number(page),
        per_page: Number(PAGE_SIZE),
        company_id: user?.company_id,
        ...(debouncedSearch?.trim() && { search: debouncedSearch }),
      }),
  });

  const createMutation = useMutation({
    mutationFn: (v: any) =>
      customersApi.create({ ...v, company_id: user?.company_id }),
    onSuccess: (res) => {
      notifications.show({
        title: "Created",
        message: "Customer created.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["customers"] });
      close();
      navigate(`/customers/${res.data.id}`);
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => customersApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Customer deleted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["customers"] });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Customers"
        description="Manage customer accounts"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Customers" }]}
        action={{
          label: "Add Customer",
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
          { accessor: "name", title: "Customer Name" },
          { accessor: "email", title: "Email" },
          { accessor: "phone", title: "Phone" },
          {
            accessor: "credit_limit",
            title: "Credit Limit",
            render: (c) => formatCurrency(c.credit_limit),
          },
          {
            accessor: "current_balance",
            title: "Balance",
            render: (c) => (
              <Text c={c.current_balance > 0 ? "red" : "green"} size="sm">
                {formatCurrency(c.current_balance)}
              </Text>
            ),
          },
          {
            accessor: "is_active",
            title: "Status",
            render: (c) => (
              <Badge color={c.is_active ? "green" : "gray"} variant="light">
                {c.is_active ? "Active" : "Inactive"}
              </Badge>
            ),
          },
          {
            accessor: "actions",
            title: "",
            width: 80,
            render: (c: Customer) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="View">
                  <ActionIcon
                    variant="subtle"
                    onClick={() => navigate(`/customers/${c.id}`)}
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
                        title: "Delete Customer",
                        message: `Delete customer "${c.name}"?`,
                        danger: true,
                        onConfirm: () => deleteMutation.mutate(c.id),
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

      <Modal opened={opened} onClose={close} title="Create Customer" size="md">
        <CustomerForm
          onSubmit={async (v) => {
            await createMutation.mutateAsync(v);
          }}
          loading={createMutation.isPending}
        />
      </Modal>
    </Stack>
  );
}
