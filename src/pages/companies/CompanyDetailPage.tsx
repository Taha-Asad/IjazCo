import { Stack, Card, Group, Text, Skeleton, Tabs } from "@mantine/core";
import { useParams } from "react-router-dom";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { PageHeader } from "../..//components/common/PageHeader";
import { CompanyForm } from "../../components/forms/CompanyForm";
import { companiesApi } from "../../api/companies";
import { formatDate } from "../../utils/formatters";

export function CompanyDetailPage() {
  const { id } = useParams<{ id: string }>();
  const queryClient = useQueryClient();

  const { data, isLoading } = useQuery({
    queryKey: ["company", id],
    queryFn: () => companiesApi.getById(id!),
    enabled: !!id,
  });

  const updateMutation = useMutation({
    mutationFn: (values: any) => companiesApi.update(id!, values),
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "Company updated.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["company", id] });
    },
  });

  const company = data?.data;
  if (isLoading) return <Skeleton height={400} />;
  if (!company) return <Text>Company not found.</Text>;

  return (
    <Stack>
      <PageHeader
        title={company.name}
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Companies", path: "/companies" },
          { label: company.name },
        ]}
      />
      <Card withBorder radius="md" p="lg">
        <Tabs defaultValue="details">
          <Tabs.List>
            <Tabs.Tab value="details">Details</Tabs.Tab>
            <Tabs.Tab value="edit">Edit</Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value="details" pt="md">
            <Stack gap="sm">
              {[
                ["Email", company.email],
                ["Phone", company.phone],
                ["City", company.city],
                ["Country", company.country],
                ["Address", company.address],
                ["Slug", company.slug],
                ["Created", formatDate(company.created_at)],
              ].map(([label, value]) => (
                <Group key={label} justify="space-between">
                  <Text c="dimmed" size="sm">
                    {label}
                  </Text>
                  <Text size="sm">{value || "—"}</Text>
                </Group>
              ))}
            </Stack>
          </Tabs.Panel>
          <Tabs.Panel value="edit" pt="md">
            <CompanyForm
              initialValues={company}
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
