import { useState } from "react";
import {
  Stack,
  Tabs,
  Card,
  Group,
  Button,
  Select,
  SimpleGrid,
  Text,
  Title,
  Skeleton,
} from "@mantine/core";
import { DatePickerInput } from "@mantine/dates";
import { useQuery } from "@tanstack/react-query";
import { BarChart } from "@mantine/charts";
import { notifications } from "@mantine/notifications";
import { IconFileExport, IconChartBar, IconPackage } from "@tabler/icons-react";
import { PageHeader } from "@/components/common/PageHeader";
import { StatCard } from "@/components/common/StatCard";
import { reportsApi } from "@/api/reports";
import { formatCurrency, formatDate } from "@/utils/formatters";
import dayjs from "dayjs";

export function ReportsPage() {
  const [startDate, setStartDate] = useState<string | null>(
    dayjs().startOf("month").format("YYYY-MM-DD"),
  );
  const [endDate, setEndDate] = useState<string | null>(
    dayjs().format("YYYY-MM-DD"),
  );
  const [exportLoading, setExportLoading] = useState(false);

  const {
    data: salesData,
    isLoading: salesLoading,
    refetch: refetchSales,
  } = useQuery({
    queryKey: ["sales-report", startDate, endDate],
    queryFn: () =>
      reportsApi.salesReport({
        start_date: startDate!,
        end_date: endDate!,
      }),
    enabled: !!startDate && !!endDate,
  });

  const { data: inventoryData, isLoading: inventoryLoading } = useQuery({
    queryKey: ["inventory-report"],
    queryFn: () => reportsApi.inventoryReport(),
  });

  const handleExportPdf = async (type: "sales" | "inventory") => {
    setExportLoading(true);
    try {
      const blob = (await reportsApi.exportPdf({
        report_type: type,
        params: {
          start_date: startDate!,
          end_date: endDate!,
        },
      })) as Blob;

      const url = window.URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = url;
      link.download = `${type}-report-${dayjs().format("YYYY-MM-DD")}.pdf`;
      link.click();
      window.URL.revokeObjectURL(url);

      notifications.show({
        title: "Exported",
        message: "Report exported successfully.",
        color: "green",
      });
    } catch {
      notifications.show({
        title: "Error",
        message: "Failed to export report.",
        color: "red",
      });
    } finally {
      setExportLoading(false);
    }
  };

  return (
    <Stack>
      <PageHeader
        title="Reports"
        description="Generate and export business reports"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Reports" }]}
      />

      <Tabs defaultValue="sales">
        <Tabs.List>
          <Tabs.Tab value="sales" leftSection={<IconChartBar size={16} />}>
            Sales Report
          </Tabs.Tab>
          <Tabs.Tab value="inventory" leftSection={<IconPackage size={16} />}>
            Inventory Report
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="sales" pt="md">
          <Stack>
            <Card withBorder radius="md" p="md">
              <Group justify="space-between" wrap="wrap">
                <Group>
                  <DatePickerInput
                    label="Start Date"
                    value={startDate}
                    onChange={setStartDate}
                    valueFormat="MMM DD, YYYY"
                    w={200}
                  />
                  <DatePickerInput
                    label="End Date"
                    value={endDate}
                    onChange={setEndDate}
                    valueFormat="MMM DD, YYYY"
                    w={200}
                  />
                  <Button mt={24} onClick={() => refetchSales()}>
                    Generate
                  </Button>
                </Group>
                <Button
                  mt={24}
                  variant="light"
                  leftSection={<IconFileExport size={16} />}
                  loading={exportLoading}
                  onClick={() => handleExportPdf("sales")}
                >
                  Export PDF
                </Button>
              </Group>
            </Card>

            {salesLoading ? (
              <Skeleton height={300} />
            ) : salesData?.data ? (
              <>
                <SimpleGrid cols={{ base: 1, sm: 3 }}>
                  <StatCard
                    title="Total Revenue"
                    value={formatCurrency(salesData.data.total_revenue || 0)}
                    icon={<IconChartBar size={20} />}
                    color="green"
                  />
                  <StatCard
                    title="Total Invoices"
                    value={salesData.data.total_invoices || 0}
                    icon={<IconChartBar size={20} />}
                    color="blue"
                  />
                  <StatCard
                    title="Average Invoice"
                    value={formatCurrency(salesData.data.avg_invoice || 0)}
                    icon={<IconChartBar size={20} />}
                    color="violet"
                  />
                </SimpleGrid>
              </>
            ) : null}
          </Stack>
        </Tabs.Panel>

        <Tabs.Panel value="inventory" pt="md">
          <Stack>
            <Card withBorder radius="md" p="md">
              <Group justify="space-between">
                <Title order={4}>Inventory Summary</Title>
                <Button
                  variant="light"
                  leftSection={<IconFileExport size={16} />}
                  loading={exportLoading}
                  onClick={() => handleExportPdf("inventory")}
                >
                  Export PDF
                </Button>
              </Group>
            </Card>

            {inventoryLoading ? (
              <Skeleton height={300} />
            ) : (
              <Card withBorder radius="md" p="md">
                <Text c="dimmed">Inventory report data will appear here.</Text>
              </Card>
            )}
          </Stack>
        </Tabs.Panel>
      </Tabs>
    </Stack>
  );
}
