import { BarChart } from "@mantine/charts";
import { Card, Title, Skeleton, Select, Group } from "@mantine/core";
import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { stockApi } from "../../api/stock";

export function StockMovementChart({ itemId }: { itemId?: string }) {
  const [movementType, setMovementType] = useState<string | null>(null);

  const { data, isLoading } = useQuery({
    queryKey: ["stock-movements-chart", itemId, movementType],
    queryFn: () =>
      stockApi.listMovements({
        item_id: itemId,
        per_page: 30,
      }),
  });

  const movements = data?.data || [];

  const chartData = movements.map((m) => ({
    date: new Date(m.created_at).toLocaleDateString(),
    in: m.movement_type === "in" ? m.quantity : 0,
    out: m.movement_type === "out" ? m.quantity : 0,
    adjustment: m.movement_type === "adjustment" ? m.quantity : 0,
  }));

  return (
    <Card withBorder radius="md" p="lg" style={{ minWidth: 0 }}>
      <Group justify="space-between" mb="md">
        <Title order={4}>Stock Movements</Title>
        <Select
          size="xs"
          placeholder="All types"
          clearable
          data={[
            { value: "in", label: "In" },
            { value: "out", label: "Out" },
            { value: "adjustment", label: "Adjustment" },
            { value: "transfer", label: "Transfer" },
          ]}
          value={movementType}
          onChange={setMovementType}
          w={130}
        />
      </Group>
      {isLoading ? (
        <Skeleton height={200} />
      ) : (
        <BarChart
          h={200}
          data={chartData}
          dataKey="date"
          series={[
            { name: "in", color: "green.6", label: "In" },
            { name: "out", color: "red.6", label: "Out" },
            { name: "adjustment", color: "orange.6", label: "Adjustment" },
          ]}
          withLegend
          withTooltip
          type="stacked"
        />
      )}
    </Card>
  );
}
