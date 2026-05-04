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
import { SupplierForm } from "../../components/forms/SupplierForm";
import { StatCard } from "../../components/common/StatCard";
import { suppliersApi } from "../../api/suppliers";
import { formatCurrency, formatDate } from "../../utils/formatters";
import { IconShoppingCart, IconCurrencyDollar } from "@tabler/icons-react";

export function SupplierDetailPage() {
  const { id } = useParams<{ id: string }>();
  const queryClient = useQueryClient();

  const { data: supplierData, isLoading } = useQuery({
    queryKey: ["supplier", id],
    queryFn: () => suppliersApi.getById(id!),
    enabled: !!id,
  });

  const updateMutation = useMutation({
    mutationFn: (v: any) => suppliersApi.update(id!, v),
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "Supplier updated.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["supplier", id] });
    },
  });

  const supplier = supplierData;
  if (isLoading) return <Skeleton height={400} />;
  if (!supplier) return <Text>Supplier not found.</Text>;

  return (
    <Stack>
      <PageHeader
        title={supplier.name}
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Suppliers", path: "/suppliers" },
          { label: supplier.name },
        ]}
      />

      <SimpleGrid cols={{ base: 1, sm: 2 }}>
        <StatCard
          title="Total Orders"
          value={supplier.total_orders || 0}
          icon={<IconShoppingCart size={20} />}
          color="blue"
        />
        <StatCard
          title="Total Spent"
          value={formatCurrency(supplier.total_spent || 0)}
          icon={<IconCurrencyDollar size={20} />}
          color="green"
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
                ["Contact Person", supplier.contact_person],
                ["Email", supplier.email],
                ["Phone", supplier.phone],
                ["City", supplier.city],
                ["Country", supplier.country],
                ["Address", supplier.address],
                [
                  "Payment Terms",
                  supplier.payment_terms
                    ? `${supplier.payment_terms} days`
                    : "—",
                ],
                ["Member Since", formatDate(supplier.created_at)],
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
            <SupplierForm
              initialValues={supplier}
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
