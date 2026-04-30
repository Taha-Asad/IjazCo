import {
  Stack,
  Card,
  Group,
  Text,
  Tabs,
  SimpleGrid,
  Badge,
  Skeleton,
} from "@mantine/core";
import { useParams, useSearchParams } from "react-router-dom";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { PageHeader } from "../../components/common/PageHeader";
import { InventoryItemForm } from "../../components/forms/InventoryItemForm";
import { StatCard } from "../../components/common/StatCard";
import { StockMovementChart } from "../../components/charts/StockMovementChart";
import { inventoryApi } from "../../api/inventory";
import { formatCurrency, formatDate } from "../../utils/formatters";
import {
  IconStack2,
  IconCurrencyDollar,
  IconPackage,
} from "@tabler/icons-react";

export function InventoryDetailPage() {
  const { id } = useParams<{ id: string }>();
  const [params] = useSearchParams();
  const queryClient = useQueryClient();
  const defaultTab = params.get("edit") ? "edit" : "details";

  const { data, isLoading } = useQuery({
    queryKey: ["inventory-item", id],
    queryFn: () => inventoryApi.getById(id!),
    enabled: !!id,
  });

  const { data: stockData } = useQuery({
    queryKey: ["item-stock", id],
    queryFn: () => inventoryApi.getStock(id!),
    enabled: !!id,
  });

  const updateMutation = useMutation({
    mutationFn: (v: any) => inventoryApi.update(id!, v),
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "Item updated.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["inventory-item", id] });
    },
  });

  const item = data?.data;
  const stockLevels = stockData?.data?.stock_levels || [];
  const totalStock = stockLevels.reduce(
    (s: number, l: any) => s + l.quantity,
    0,
  );

  if (isLoading) return <Skeleton height={400} />;
  if (!item) return <Text>Item not found.</Text>;

  return (
    <Stack>
      <PageHeader
        title={item.name}
        description={item.sku}
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Inventory", path: "/inventory" },
          { label: item.name },
        ]}
      />

      <SimpleGrid cols={{ base: 1, sm: 3 }}>
        <StatCard
          title="Total Stock"
          value={`${totalStock} ${item.unit}`}
          icon={<IconStack2 size={20} />}
          color={totalStock <= item.min_stock_level ? "red" : "green"}
        />
        <StatCard
          title="Selling Price"
          value={formatCurrency(item.unit_price)}
          icon={<IconCurrencyDollar size={20} />}
          color="blue"
        />
        <StatCard
          title="Cost Price"
          value={formatCurrency(item.cost_price)}
          icon={<IconCurrencyDollar size={20} />}
          color="orange"
        />
      </SimpleGrid>

      <Card withBorder radius="md" p="lg">
        <Tabs defaultValue={defaultTab}>
          <Tabs.List>
            <Tabs.Tab value="details">Details</Tabs.Tab>
            <Tabs.Tab value="stock">Stock Levels</Tabs.Tab>
            <Tabs.Tab value="movements">Movements</Tabs.Tab>
            <Tabs.Tab value="edit">Edit</Tabs.Tab>
          </Tabs.List>

          <Tabs.Panel value="details" pt="md">
            <Stack gap="sm">
              {[
                ["SKU", item.sku],
                ["Category", item.category_name || "—"],
                ["Unit", item.unit],
                ["Serial Number", item.serial_number || "—"],
                ["Min Stock", `${item.min_stock_level} ${item.unit}`],
                ["Description", item.description || "—"],
                ["Created", formatDate(item.created_at)],
              ].map(([label, value]) => (
                <Group key={label as string} justify="space-between">
                  <Text c="dimmed" size="sm">
                    {label}
                  </Text>
                  <Text size="sm">{value}</Text>
                </Group>
              ))}
            </Stack>
          </Tabs.Panel>

          <Tabs.Panel value="stock" pt="md">
            <Stack gap="xs">
              {stockLevels.length === 0 ? (
                <Text c="dimmed">No stock data available.</Text>
              ) : (
                stockLevels.map((level: any) => (
                  <Group
                    key={level.branch_id}
                    justify="space-between"
                    p="sm"
                    style={{
                      background: "var(--mantine-color-gray-0)",
                      borderRadius: 8,
                    }}
                  >
                    <Text size="sm">{level.branch_name}</Text>
                    <Badge
                      color={
                        level.quantity <= item.min_stock_level ? "red" : "green"
                      }
                      variant="light"
                    >
                      {level.quantity} {item.unit}
                    </Badge>
                  </Group>
                ))
              )}
            </Stack>
          </Tabs.Panel>

          <Tabs.Panel value="movements" pt="md">
            <StockMovementChart itemId={id} />
          </Tabs.Panel>

          <Tabs.Panel value="edit" pt="md">
            <InventoryItemForm
              initialValues={item}
              onSubmit={async (v) => {
                await updateMutation.mutateAsync(v);
              }}
              loading={updateMutation.isPending}
            />
          </Tabs.Panel>
        </Tabs>
      </Card>
    </Stack>
  );
}
