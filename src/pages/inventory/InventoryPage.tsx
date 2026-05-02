import { useState } from "react";
import { useNavigate } from "react-router-dom";
import {
  Stack,
  TextInput,
  Group,
  Button,
  Badge,
  ActionIcon,
  Tooltip,
} from "@mantine/core";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import {
  IconSearch,
  IconPlus,
  IconEdit,
  IconTrash,
  IconEye,
  IconPackage,
} from "@tabler/icons-react";
import { PageHeader } from "@/components/common/PageHeader";
import { openConfirmModal } from "@/components/common/ConfirmModal";
import { inventoryApi } from "@/api/inventory";
import { formatCurrency } from "@/utils/formatters";
import { useDebounce } from "@/hooks/useDebounce";
import { useAuthStore } from "@/store/authStore";
import type { InventoryItem } from "@/types";

const PAGE_SIZE = 20;

export function InventoryPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { user } = useAuthStore();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["inventory", page, debouncedSearch],
    queryFn: () =>
      inventoryApi.list({
        page: Number(page),
        per_page: Number(PAGE_SIZE),
        company_id: user?.company_id,
        ...(debouncedSearch?.trim() && { search: debouncedSearch }),
      }),
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => inventoryApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Item deleted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["inventory"] });
    },
    onError: () => {
      notifications.show({
        title: "Error",
        message: "Failed to delete item.",
        color: "red",
      });
    },
  });

  const handleDelete = (item: InventoryItem) => {
    openConfirmModal({
      title: "Delete Item",
      message: `Are you sure you want to delete "${item.name}"? This cannot be undone.`,
      confirmLabel: "Delete",
      danger: true,
      onConfirm: () => deleteMutation.mutate(item.id),
    });
  };

  return (
    <Stack>
      <PageHeader
        title="Inventory"
        description="Manage your inventory items and stock levels"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Inventory" }]}
        action={{
          label: "Add Item",
          icon: <IconPlus size={16} />,
          onClick: () => navigate("/inventory/create"),
        }}
      />

      <Group justify="space-between">
        <TextInput
          placeholder="Search items..."
          leftSection={<IconSearch size={16} />}
          value={search}
          onChange={(e) => setSearch(e.currentTarget.value)}
          w={300}
        />
        <Button
          variant="light"
          color="orange"
          leftSection={<IconPackage size={16} />}
          onClick={() => navigate("/inventory/low-stock")}
        >
          Low Stock Alert
        </Button>
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
          {
            accessor: "sku",
            title: "SKU",
            width: 120,
            render: (item) => (
              <Badge variant="outline" size="sm">
                {item.sku}
              </Badge>
            ),
          },
          { accessor: "name", title: "Item Name" },
          { accessor: "category_name", title: "Category" },
          { accessor: "unit", title: "Unit", width: 80 },
          {
            accessor: "unit_price",
            title: "Unit Price",
            render: (item) => formatCurrency(item.unit_price),
          },
          {
            accessor: "total_stock",
            title: "Stock",
            render: (item) => (
              <Badge
                color={
                  (item.total_stock || 0) === 0
                    ? "red"
                    : (item.total_stock || 0) <= item.min_stock_level
                      ? "orange"
                      : "green"
                }
                variant="light"
              >
                {item.total_stock || 0} {item.unit}
              </Badge>
            ),
          },
          {
            accessor: "actions",
            title: "",
            width: 100,
            render: (item) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="View">
                  <ActionIcon
                    variant="subtle"
                    onClick={() => navigate(`/inventory/${item.id}`)}
                  >
                    <IconEye size={16} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label="Edit">
                  <ActionIcon
                    variant="subtle"
                    color="blue"
                    onClick={() => navigate(`/inventory/${item.id}?edit=true`)}
                  >
                    <IconEdit size={16} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label="Delete">
                  <ActionIcon
                    variant="subtle"
                    color="red"
                    onClick={() => handleDelete(item)}
                    loading={deleteMutation.isPending}
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
    </Stack>
  );
}
