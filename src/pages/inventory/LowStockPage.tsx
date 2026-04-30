import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Stack, Badge, ActionIcon, Tooltip, Alert, Text } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { IconEye, IconAlertTriangle } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { inventoryApi } from "../../api/inventory";
import { formatCurrency } from "../../utils/formatters";

const PAGE_SIZE = 20;

export function LowStockPage() {
  const navigate = useNavigate();
  const [page, setPage] = useState(1);

  const { data, isLoading } = useQuery({
    queryKey: ["low-stock", page],
    queryFn: () => inventoryApi.getLowStock({ page, per_page: PAGE_SIZE }),
  });

  return (
    <Stack>
      <PageHeader
        title="Low Stock Alerts"
        description="Items below minimum stock threshold"
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Inventory", path: "/inventory" },
          { label: "Low Stock" },
        ]}
      />

      {(data?.total_items || 0) > 0 && (
        <Alert icon={<IconAlertTriangle />} color="orange" radius="md">
          {data?.total_items} item(s) are below minimum stock level and need
          restocking.
        </Alert>
      )}

      <DataTable
        records={data?.data || []}
        fetching={isLoading}
        totalRecords={data?.total_items || 0}
        recordsPerPage={PAGE_SIZE}
        page={page}
        onPageChange={setPage}
        columns={[
          {
            accessor: "sku",
            title: "SKU",
            render: (item) => (
              <Badge variant="outline" size="sm">
                {item.sku}
              </Badge>
            ),
          },
          { accessor: "name", title: "Item Name" },
          { accessor: "category_name", title: "Category" },
          {
            accessor: "total_stock",
            title: "Current Stock",
            render: (item) => (
              <Badge color="red" variant="light">
                {item.total_stock || 0} {item.unit}
              </Badge>
            ),
          },
          {
            accessor: "min_stock_level",
            title: "Min Required",
            render: (item) => `${item.min_stock_level} ${item.unit}`,
          },
          {
            accessor: "unit_price",
            title: "Unit Price",
            render: (item) => formatCurrency(item.unit_price),
          },
          {
            accessor: "actions",
            title: "",
            width: 60,
            render: (item) => (
              <Tooltip label="View Item">
                <ActionIcon
                  variant="subtle"
                  onClick={() => navigate(`/inventory/${item.id}`)}
                >
                  <IconEye size={16} />
                </ActionIcon>
              </Tooltip>
            ),
          },
        ]}
        highlightOnHover
        withTableBorder
        borderRadius="md"
        rowBackgroundColor={() => "var(--mantine-color-red-0)"}
      />
    </Stack>
  );
}
