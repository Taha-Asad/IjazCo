import { useState } from "react";
import { useNavigate } from "react-router-dom";
import {
  Stack,
  Group,
  ActionIcon,
  Tooltip,
  Select,
  Badge,
} from "@mantine/core";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import {
  IconPlus,
  IconEye,
  IconTrash,
  IconSend,
  IconPackageImport,
} from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { SearchInput } from "../../components/common/SearchInput";
import { StatusBadge } from "../../components/common/StatusBadge";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { purchasesApi, type PurchaseOrder } from "../../api/purchases";
import { useDebounce } from "../../hooks/useDebounce";
import { formatCurrency, formatDate } from "../../utils/formatters";
import { useAuthStore } from "../../store/authStore";

const PAGE_SIZE = 20;

export function PurchasesPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { user } = useAuthStore();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<string | null>(null);
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["purchases", page, debouncedSearch, statusFilter],
    queryFn: () =>
      purchasesApi.list({
        page: Number(page),
        per_page: Number(PAGE_SIZE),
        company_id: user?.company_id,
        ...(debouncedSearch?.trim() && { search: debouncedSearch }),
        status: statusFilter || undefined,
      }),
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => purchasesApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Purchase order deleted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["purchases"] });
    },
  });

  const submitMutation = useMutation({
    mutationFn: (id: string) => purchasesApi.submit(id),
    onSuccess: () => {
      notifications.show({
        title: "Submitted",
        message: "Purchase order submitted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["purchases"] });
    },
  });

  const receiveMutation = useMutation({
    mutationFn: (id: string) => purchasesApi.receiveGoods(id),
    onSuccess: () => {
      notifications.show({
        title: "Received",
        message: "Goods received and stock updated.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["purchases"] });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Purchase Orders"
        description="Manage supplier purchase orders"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Purchases" }]}
        action={{
          label: "Create PO",
          icon: <IconPlus size={16} />,
          onClick: () => navigate("/purchases/create"),
        }}
      />
      <Group>
        <SearchInput value={search} onChange={setSearch} w={260} />
        <Select
          placeholder="Filter by status"
          clearable
          value={statusFilter}
          onChange={setStatusFilter}
          data={[
            { value: "draft", label: "Draft" },
            { value: "submitted", label: "Submitted" },
            { value: "received", label: "Received" },
            { value: "cancelled", label: "Cancelled" },
          ]}
          w={160}
        />
      </Group>
      <DataTable
        records={data?.data || []}
        fetching={isLoading}
        totalRecords={data?.total_items || 0}
        recordsPerPage={PAGE_SIZE}
        page={page}
        onPageChange={setPage}
        columns={[
          { accessor: "po_number", title: "PO Number" },
          { accessor: "supplier_name", title: "Supplier" },
          {
            accessor: "total_amount",
            title: "Total",
            render: (po) => formatCurrency(po.total_amount),
          },
          {
            accessor: "status",
            title: "Status",
            render: (po) => <StatusBadge status={po.status} />,
          },
          {
            accessor: "expected_date",
            title: "Expected",
            render: (po) =>
              po.expected_date ? formatDate(po.expected_date) : "—",
          },
          {
            accessor: "created_at",
            title: "Created",
            render: (po) => formatDate(po.created_at),
          },
          {
            accessor: "actions",
            title: "",
            width: 130,
            render: (po: PurchaseOrder) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="View">
                  <ActionIcon
                    variant="subtle"
                    onClick={() => navigate(`/purchases/${po.id}`)}
                  >
                    <IconEye size={16} />
                  </ActionIcon>
                </Tooltip>
                {po.status === "draft" && (
                  <Tooltip label="Submit">
                    <ActionIcon
                      variant="subtle"
                      color="blue"
                      onClick={() =>
                        openConfirmModal({
                          title: "Submit PO",
                          message: "Submit this purchase order?",
                          onConfirm: () => submitMutation.mutate(po.id),
                        })
                      }
                    >
                      <IconSend size={16} />
                    </ActionIcon>
                  </Tooltip>
                )}
                {po.status === "submitted" && (
                  <Tooltip label="Receive Goods">
                    <ActionIcon
                      variant="subtle"
                      color="green"
                      onClick={() =>
                        openConfirmModal({
                          title: "Receive Goods",
                          message:
                            "Mark goods as received? This will add stock.",
                          onConfirm: () => receiveMutation.mutate(po.id),
                        })
                      }
                    >
                      <IconPackageImport size={16} />
                    </ActionIcon>
                  </Tooltip>
                )}
                {po.status === "draft" && (
                  <Tooltip label="Delete">
                    <ActionIcon
                      variant="subtle"
                      color="red"
                      onClick={() =>
                        openConfirmModal({
                          title: "Delete PO",
                          message: `Delete PO "${po.po_number}"?`,
                          danger: true,
                          onConfirm: () => deleteMutation.mutate(po.id),
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
