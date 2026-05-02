import { AreaChart, BarChart } from "@mantine/charts";
import { Card, Title, Group, SegmentedControl } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { dashboardApi } from "../../api/dashboard";
import { useState } from "react";

export function SalesChart() {
  const [chartType, setChartType] = useState<"area" | "bar">("area");
  const [period, setPeriod] = useState<"daily" | "monthly">("monthly");

  const { data, isLoading } = useQuery({
    queryKey: ["sales-chart", period],
    queryFn: () => dashboardApi.getSalesChart({ period, months: 12 }),
  });

  const chartData = data?.data || [];

  return (
    <Card withBorder radius="md" p="lg" style={{ minWidth: 0 }}>
      <Group justify="space-between" mb="md">
        <Title order={4}>Sales Trend</Title>
        <Group gap="xs">
          <SegmentedControl
            size="xs"
            data={[
              { value: "monthly", label: "Monthly" },
              { value: "daily", label: "Daily" },
            ]}
            value={period}
            onChange={(v) => setPeriod(v as "daily" | "monthly")}
          />
          <SegmentedControl
            size="xs"
            data={[
              { value: "area", label: "Area" },
              { value: "bar", label: "Bar" },
            ]}
            value={chartType}
            onChange={(v) => setChartType(v as "area" | "bar")}
          />
        </Group>
      </Group>

      {chartType === "area" ? (
        <AreaChart
          h={280}
          data={chartData}
          dataKey="period"
          series={[
            { name: "revenue", color: "green.6", label: "Revenue ($)" },
            { name: "invoices", color: "blue.6", label: "Invoices" },
          ]}
          curveType="natural"
          withLegend
          withTooltip
          withDots={false}
        />
      ) : (
        <BarChart
          h={280}
          data={chartData}
          dataKey="period"
          series={[{ name: "revenue", color: "green.6", label: "Revenue ($)" }]}
          withLegend
          withTooltip
        />
      )}
    </Card>
  );
}
