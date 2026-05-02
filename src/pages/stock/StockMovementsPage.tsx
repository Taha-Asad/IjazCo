import { useState } from "react";
import { Stack, Badge, Select, Group, Text } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { PageHeader } from "../../components/common/PageHeader";
import { SearchInput } from "../../components/common/SearchInput";
import { stockApi } from "../../api/stock";
import { useDebounce } from "../../hooks/useDebounce";
import { formatDateTime } from "../../utils/formatters";
import { useAuthStore } from "../../store/authStore";

const PAGE_SIZE = 30;

const MOVEMENT_COLORS: Record<string, string> = {
  in: "green",
  out: "red",
  transfer: "blue",
  adjustment: "orange",
  count: "violet",
};

export function StockMovementsPage() {
  const { user } = useAuthStore();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [typeFilter, setTypeFilter] = useState<string | null>(null);
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["stock-movements", page, debouncedSearch, typeFilter],
    queryFn: () =>
      stockApi.listMovements({
        page: Number(page),
        per_page: Number(PAGE_SIZE),
        company_id: user?.company_id,
      }),
  });

  return (
    <Stack>
      <PageHeader
        title="Stock Movements"
        description="Full audit trail of all stock changes"
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Stock", path: "/stock" },
          { label: "Movements" },
        ]}
      />
      <Group>
        <SearchInput value={search} onChange={setSearch} w={260} />
        <Select
          placeholder="Filter by type"
          clearable
          value={typeFilter}
          onChange={setTypeFilter}
          data={[
            { value: "in", label: "In" },
            { value: "out", label: "Out" },
            { value: "transfer", label: "Transfer" },
            { value: "adjustment", label: "Adjustment" },
            { value: "count", label: "Physical Count" },
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
          { accessor: "item_name", title: "Item" },
          { accessor: "branch_name", title: "Branch" },
          {
            accessor: "movement_type",
            title: "Type",
            render: (m) => (
              <Badge
                color={MOVEMENT_COLORS[m.movement_type] || "gray"}
                variant="light"
              >
                {m.movement_type.toUpperCase()}
              </Badge>
            ),
          },
          {
            accessor: "quantity",
            title: "Quantity",
            render: (m) => (
              <Text
                fw={600}
                c={
                  m.movement_type === "in"
                    ? "green"
                    : m.movement_type === "out"
                      ? "red"
                      : "gray"
                }
              >
                {m.movement_type === "in"
                  ? "+"
                  : m.movement_type === "out"
                    ? "-"
                    : ""}
                {Math.abs(m.quantity)}
              </Text>
            ),
          },
          {
            accessor: "reference",
            title: "Reference",
            render: (m) => m.reference || "—",
          },
          { accessor: "notes", title: "Notes", render: (m) => m.notes || "—" },
          {
            accessor: "created_at",
            title: "Date",
            render: (m) => formatDateTime(m.created_at),
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
