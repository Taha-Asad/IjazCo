import { useState } from "react";
import { Stack, Group, Badge, ActionIcon, Tooltip, Modal } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import { IconAdjustments, IconArrowsLeftRight } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { SearchInput } from "../../components/common/SearchInput";
import { StockAdjustForm } from "../../components/forms/StockAdjustForm";
import { StockTransferForm } from "../../components/forms/StockTransferForm";
import { stockApi, type StockRecord } from "../../api/stock";
import { useDebounce } from "../../hooks/useDebounce";

const PAGE_SIZE = 20;

export function StockPage() {
  const queryClient = useQueryClient();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [selected, setSelected] = useState<StockRecord | null>(null);
  const [adjustOpened, { open: openAdjust, close: closeAdjust }] =
    useDisclosure(false);
  const [transferOpened, { open: openTransfer, close: closeTransfer }] =
    useDisclosure(false);
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["stock", page, debouncedSearch],
    queryFn: () =>
      stockApi.list({ page, per_page: PAGE_SIZE, search: debouncedSearch }),
  });

  const adjustMutation = useMutation({
    mutationFn: stockApi.adjust,
    onSuccess: () => {
      notifications.show({
        title: "Adjusted",
        message: "Stock adjusted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["stock"] });
      closeAdjust();
    },
  });

  const transferMutation = useMutation({
    mutationFn: stockApi.transfer,
    onSuccess: () => {
      notifications.show({
        title: "Transferred",
        message: "Stock transferred.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["stock"] });
      closeTransfer();
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Stock Levels"
        description="View and manage stock across all branches"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Stock" }]}
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
          {
            accessor: "item_sku",
            title: "SKU",
            render: (s) => (
              <Badge variant="outline" size="sm">
                {s.item_sku}
              </Badge>
            ),
          },
          { accessor: "item_name", title: "Item Name" },
          { accessor: "branch_name", title: "Branch" },
          {
            accessor: "quantity",
            title: "Quantity",
            render: (s) => (
              <Badge
                color={
                  s.quantity === 0
                    ? "red"
                    : s.quantity <= s.min_stock_level
                      ? "orange"
                      : "green"
                }
                variant="light"
              >
                {s.quantity} {s.unit}
              </Badge>
            ),
          },
          {
            accessor: "min_stock_level",
            title: "Min Level",
            render: (s) => `${s.min_stock_level} ${s.unit}`,
          },
          {
            accessor: "actions",
            title: "",
            width: 100,
            render: (s: StockRecord) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="Adjust Stock">
                  <ActionIcon
                    variant="subtle"
                    color="blue"
                    onClick={() => {
                      setSelected(s);
                      openAdjust();
                    }}
                  >
                    <IconAdjustments size={16} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label="Transfer Stock">
                  <ActionIcon
                    variant="subtle"
                    color="violet"
                    onClick={() => {
                      setSelected(s);
                      openTransfer();
                    }}
                  >
                    <IconArrowsLeftRight size={16} />
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

      <Modal opened={adjustOpened} onClose={closeAdjust} title="Adjust Stock">
        {selected && (
          <StockAdjustForm
            itemId={selected.item_id}
            branchId={selected.branch_id}
            currentQuantity={selected.quantity}
            onSubmit={async (v) => {
              await adjustMutation.mutateAsync(v);
            }}
            loading={adjustMutation.isPending}
          />
        )}
      </Modal>

      <Modal
        opened={transferOpened}
        onClose={closeTransfer}
        title="Transfer Stock"
      >
        {selected && (
          <StockTransferForm
            itemId={selected.item_id}
            currentBranchId={selected.branch_id}
            currentQuantity={selected.quantity}
            branches={[]}
            onSubmit={async (v) => {
              await transferMutation.mutateAsync(v);
            }}
            loading={transferMutation.isPending}
          />
        )}
      </Modal>
    </Stack>
  );
}
