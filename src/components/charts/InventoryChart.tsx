import { DonutChart } from "@mantine/charts";
import { Card, Title, Stack, Group, Text, Skeleton } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { dashboardApi } from "../../api/dashboard";
import { formatCurrency } from "../../utils/formatters";

const COLORS = [
  "blue.6",
  "green.6",
  "orange.6",
  "violet.6",
  "red.6",
  "cyan.6",
  "yellow.6",
  "pink.6",
];

export function InventoryChart() {
  const { data, isLoading } = useQuery({
    queryKey: ["inventory-valuation"],
    queryFn: () => dashboardApi.getInventoryValuation(),
  });

  const valuations = data?.data || [];
  const chartData = valuations.map((v, i) => ({
    name: v.branch_name,
    value: v.total_value,
    color: COLORS[i % COLORS.length],
  }));

  const totalValue = valuations.reduce((s, v) => s + v.total_value, 0);

  return (
    <Card withBorder radius="md" p="lg" style={{ minWidth: 0 }}>
      <Title order={4} mb="md">
        Inventory Valuation by Branch
      </Title>
      {isLoading ? (
        <Skeleton height={200} />
      ) : (
        <Group justify="space-between" align="flex-start">
          <DonutChart
            data={chartData}
            size={160}
            thickness={30}
            withTooltip
            tooltipDataSource="segment"
          />
          <Stack gap="xs" style={{ flex: 1 }}>
            {valuations.map((v, i) => (
              <Group key={v.branch_id} justify="space-between">
                <Group gap="xs">
                  <div
                    style={{
                      width: 12,
                      height: 12,
                      borderRadius: 4,
                      background: `var(--mantine-color-${COLORS[i % COLORS.length].replace(".", "-")})`,
                    }}
                  />
                  <Text size="sm">{v.branch_name}</Text>
                </Group>
                <Text size="sm" fw={600}>
                  {formatCurrency(v.total_value)}
                </Text>
              </Group>
            ))}
            <Group justify="space-between" mt="xs">
              <Text size="sm" fw={700}>
                Total
              </Text>
              <Text size="sm" fw={700}>
                {formatCurrency(totalValue)}
              </Text>
            </Group>
          </Stack>
        </Group>
      )}
    </Card>
  );
}
