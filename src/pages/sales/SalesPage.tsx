import { useState } from "react";
import { useNavigate } from "react-router-dom";
import {
  Stack,
  TextInput,
  Group,
  Button,
  Select,
  ActionIcon,
  Tooltip,
  Text,
} from "@mantine/core";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import {
  IconSearch,
  IconPlus,
  IconEye,
  IconTrash,
  IconCheck,
} from "@tabler/icons-react";
import { useDebounce } from "../../hooks/useDebounce";
import { salesApi } from "../../api/sales";
import { PageHeader } from "../../components/common/PageHeader";
import { formatCurrency, formatDate } from "../../utils/formatters";
import { StatusBadge } from "../../components/common/StatusBadge";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { useAuthStore } from "../../store/authStore";
import { SalesInvoice } from "../../types";

const PAGE_SIZE = 20;

export function SalesPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { user } = useAuthStore();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<string | null>(null);
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["sales", page, debouncedSearch, statusFilter],
    queryFn: () =>
      salesApi.list({
        page: Number(page),
        per_page: Number(PAGE_SIZE),
        company_id: user?.company_id,
        ...(debouncedSearch?.trim() && { search: debouncedSearch }),
        status: statusFilter || undefined,
      }),
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => salesApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Invoice deleted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["sales"] });
    },
  });

  const approveMutation = useMutation({
    mutationFn: (id: string) => salesApi.approve(id),
    onSuccess: () => {
      notifications.show({
        title: "Approved",
        message: "Invoice approved and stock deducted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["sales"] });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Sales Invoices"
        description="Manage sales invoices and payments"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Sales" }]}
        action={{
          label: "Create Invoice",
          icon: <IconPlus size={16} />,
          onClick: () => navigate("/sales/create"),
        }}
      />

      <Group justify="space-between">
        <Group>
          <TextInput
            placeholder="Search invoices..."
            leftSection={<IconSearch size={16} />}
            value={search}
            onChange={(e) => setSearch(e.currentTarget.value)}
            w={250}
          />
          <Select
            placeholder="Filter by status"
            clearable
            value={statusFilter}
            onChange={setStatusFilter}
            data={[
              { value: "draft", label: "Draft" },
              { value: "approved", label: "Approved" },
              { value: "paid", label: "Paid" },
              { value: "partial", label: "Partial" },
              { value: "cancelled", label: "Cancelled" },
            ]}
            w={160}
          />
        </Group>
      </Group>

      <DataTable
        records={data?.data || []}
        fetching={isLoading}
        totalRecords={data?.total_items || 0}
        recordsPerPage={PAGE_SIZE}
        page={page}
        onPageChange={setPage}
        minHeight={400}
        columns={[
          { accessor: "invoice_number", title: "Invoice #" },
          { accessor: "customer_name", title: "Customer" },
          {
            accessor: "total_amount",
            title: "Total",
            render: (inv) => formatCurrency(inv.total_amount),
          },
          {
            accessor: "due_amount",
            title: "Balance",
            render: (inv) => (
              <Text c={inv.due_amount > 0 ? "red" : "green"} fw={500} size="sm">
                {formatCurrency(inv.due_amount)}
              </Text>
            ),
          },
          {
            accessor: "status",
            title: "Status",
            render: (inv) => <StatusBadge status={inv.status} />,
          },
          {
            accessor: "created_at",
            title: "Date",
            render: (inv) => formatDate(inv.created_at),
          },
          {
            accessor: "actions",
            title: "",
            width: 120,
            render: (inv: SalesInvoice) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="View">
                  <ActionIcon
                    variant="subtle"
                    onClick={() => navigate(`/sales/${inv.id}`)}
                  >
                    <IconEye size={16} />
                  </ActionIcon>
                </Tooltip>
                {inv.status === "draft" && (
                  <Tooltip label="Approve">
                    <ActionIcon
                      variant="subtle"
                      color="green"
                      onClick={() =>
                        openConfirmModal({
                          title: "Approve Invoice",
                          message: "Approving will deduct stock. Continue?",
                          confirmLabel: "Approve",
                          onConfirm: () => approveMutation.mutate(inv.id),
                        })
                      }
                    >
                      <IconCheck size={16} />
                    </ActionIcon>
                  </Tooltip>
                )}
                {inv.status === "draft" && (
                  <Tooltip label="Delete">
                    <ActionIcon
                      variant="subtle"
                      color="red"
                      onClick={() =>
                        openConfirmModal({
                          title: "Delete Invoice",
                          message: `Delete invoice "${inv.invoice_number}"?`,
                          confirmLabel: "Delete",
                          danger: true,
                          onConfirm: () => deleteMutation.mutate(inv.id),
                        })
                      }
                    >
                      <IconTrash size={16} />
                    </ActionIcon>
                  </Tooltip>
                )}
              </Group>
            ),
          },
        ]}
        highlightOnHover
        withTableBorder
        borderRadius="md"
        striped
      />
    </Stack>
  );
}
