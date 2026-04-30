import {
  Grid,
  SimpleGrid,
  Card,
  Title,
  Text,
  Group,
  Stack,
  Skeleton,
} from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import {
  IconCurrencyDollar,
  IconReceipt,
  IconUsers,
  IconAlertTriangle,
} from "@tabler/icons-react";
import { AreaChart } from "@mantine/charts";
import { StatCard } from "../../components/common/StatCard";
import { dashboardApi } from "../../api/dashboard";
import { formatCurrency } from "../../utils/formatters";
import { StatusBadge } from "../../components/common/StatusBadge";

export function DashboardPage() {
  const { data: statsData, isLoading: statsLoading } = useQuery({
    queryKey: ["dashboard-stats"],
    queryFn: () => dashboardApi.getStats(),
  });

  const { data: chartData, isLoading: chartLoading } = useQuery({
    queryKey: ["sales-chart"],
    queryFn: () => dashboardApi.getSalesChart({ period: "monthly", months: 6 }),
  });

  const stats = statsData?.data;
  const chart = chartData?.data || [];

  return (
    <Stack>
      <div>
        <Title order={2}>Dashboard</Title>
        <Text c="dimmed">Welcome to your ERP overview</Text>
      </div>

      {/* Stat Cards */}
      <SimpleGrid cols={{ base: 1, sm: 2, lg: 4 }}>
        <StatCard
          title="Total Revenue"
          value={formatCurrency(stats?.total_revenue || 0)}
          icon={<IconCurrencyDollar size={20} />}
          change={stats?.revenue_change}
          changeLabel="vs last month"
          color="green"
          loading={statsLoading}
        />
        <StatCard
          title="Total Invoices"
          value={stats?.total_invoices || 0}
          icon={<IconReceipt size={20} />}
          change={stats?.invoices_change}
          changeLabel="vs last month"
          color="blue"
          loading={statsLoading}
        />
        <StatCard
          title="Total Customers"
          value={stats?.total_customers || 0}
          icon={<IconUsers size={20} />}
          change={stats?.customers_change}
          changeLabel="vs last month"
          color="violet"
          loading={statsLoading}
        />
        <StatCard
          title="Low Stock Items"
          value={stats?.low_stock_items || 0}
          icon={<IconAlertTriangle size={20} />}
          color="orange"
          loading={statsLoading}
        />
      </SimpleGrid>

      {/* Charts Row */}
      <Grid>
        <Grid.Col span={{ base: 12, md: 8 }}>
          <Card withBorder radius="md" p="lg">
            <Title order={4} mb="md">
              Sales Revenue (Monthly)
            </Title>
            {chartLoading ? (
              <Skeleton height={250} />
            ) : (
              <AreaChart
                h={250}
                data={chart}
                dataKey="period"
                series={[
                  { name: "revenue", color: "green.6", label: "Revenue" },
                ]}
                curveType="natural"
                withLegend
              />
            )}
          </Card>
        </Grid.Col>

        <Grid.Col span={{ base: 12, md: 4 }}>
          <Card withBorder radius="md" p="lg" h="100%">
            <Title order={4} mb="md">
              Quick Stats
            </Title>
            <Stack>
              <Group justify="space-between">
                <Text size="sm" c="dimmed">
                  Inventory Value
                </Text>
                <Text size="sm" fw={600}>
                  {formatCurrency(stats?.total_inventory_value || 0)}
                </Text>
              </Group>
              <Group justify="space-between">
                <Text size="sm" c="dimmed">
                  Pending Orders
                </Text>
                <Text size="sm" fw={600}>
                  {stats?.pending_orders || 0}
                </Text>
              </Group>
            </Stack>
          </Card>
        </Grid.Col>
      </Grid>

      {/* Recent Sales */}
      <Card withBorder radius="md" p="lg">
        <Title order={4} mb="md">
          Recent Sales
        </Title>
        {statsLoading ? (
          <Skeleton height={200} />
        ) : (
          <Stack gap="xs">
            {stats?.recent_sales?.map((sale) => (
              <Group
                key={sale.id}
                justify="space-between"
                p="sm"
                style={{
                  borderRadius: 8,
                  background: "var(--mantine-color-gray-0)",
                }}
              >
                <div>
                  <Text size="sm" fw={500}>
                    {sale.invoice_number}
                  </Text>
                  <Text size="xs" c="dimmed">
                    {sale.customer_name}
                  </Text>
                </div>
                <Group gap="sm">
                  <StatusBadge status={sale.status} />
                  <Text size="sm" fw={600}>
                    {formatCurrency(sale.total_amount)}
                  </Text>
                </Group>
              </Group>
            ))}
          </Stack>
        )}
      </Card>
    </Stack>
  );
}
