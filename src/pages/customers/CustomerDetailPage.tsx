import {
  Stack,
  Card,
  Group,
  Text,
  Tabs,
  SimpleGrid,
  Skeleton,
} from "@mantine/core";
import { useParams } from "react-router-dom";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { PageHeader } from "../../components/common/PageHeader";
import { CustomerForm } from "../../components/forms/CustomerForm";
import { StatCard } from "../../components/common/StatCard";
import { customersApi } from "../../api/customers";
import { formatCurrency, formatDate } from "../../utils/formatters";
import { IconReceipt, IconCurrencyDollar } from "@tabler/icons-react";

export function CustomerDetailPage() {
  const { id } = useParams<{ id: string }>();
  const queryClient = useQueryClient();

  const { data, isLoading } = useQuery({
    queryKey: ["customer", id],
    queryFn: () => customersApi.getById(id!),
    enabled: !!id,
  });

  const updateMutation = useMutation({
    mutationFn: (v: any) => customersApi.update(id!, v),
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "Customer updated.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["customer", id] });
    },
  });

  const customer = data?.data;
  if (isLoading) return <Skeleton height={400} />;
  if (!customer) return <Text>Customer not found.</Text>;

  return (
    <Stack>
      <PageHeader
        title={customer.name}
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Customers", path: "/customers" },
          { label: customer.name },
        ]}
      />

      <SimpleGrid cols={{ base: 1, sm: 3 }}>
        <StatCard
          title="Total Invoices"
          value={customer.total_invoices || 0}
          icon={<IconReceipt size={20} />}
          color="blue"
        />
        <StatCard
          title="Total Spent"
          value={formatCurrency(customer.total_spent || 0)}
          icon={<IconCurrencyDollar size={20} />}
          color="green"
        />
        <StatCard
          title="Outstanding Balance"
          value={formatCurrency(customer.current_balance)}
          icon={<IconCurrencyDollar size={20} />}
          color={customer.current_balance > 0 ? "red" : "green"}
        />
      </SimpleGrid>

      <Card withBorder radius="md" p="lg">
        <Tabs defaultValue="details">
          <Tabs.List>
            <Tabs.Tab value="details">Details</Tabs.Tab>
            <Tabs.Tab value="edit">Edit</Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value="details" pt="md">
            <Stack gap="sm">
              {[
                ["Email", customer.email],
                ["Phone", customer.phone],
                ["City", customer.city],
                ["Country", customer.country],
                ["Address", customer.address],
                ["Credit Limit", formatCurrency(customer.credit_limit)],
                ["Member Since", formatDate(customer.created_at)],
              ].map(([label, value]) => (
                <Group key={label as string} justify="space-between">
                  <Text c="dimmed" size="sm">
                    {label}
                  </Text>
                  <Text size="sm">{value || "—"}</Text>
                </Group>
              ))}
            </Stack>
          </Tabs.Panel>
          <Tabs.Panel value="edit" pt="md">
            <CustomerForm
              initialValues={customer}
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
