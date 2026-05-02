import { Stack, Card, Skeleton, Text } from "@mantine/core";
import { useParams } from "react-router-dom";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { PageHeader } from "../../components/common/PageHeader";
import { RoleForm } from "../../components/forms/RoleForm";
import { rolesApi } from "../../api/roles";

export function RoleDetailPage() {
  const { id } = useParams<{ id: string }>();
  const queryClient = useQueryClient();

  const { data, isLoading } = useQuery({
    queryKey: ["role", id],
    queryFn: () => rolesApi.getById(id!),
    enabled: !!id,
  });

  const updateMutation = useMutation({
    mutationFn: async (values: any) => {
      await rolesApi.update(id!, {
        name: values.name,
        description: values.description,
      });
      await rolesApi.updatePermissions(id!, {
        permissions: values.permissions,
      });
    },
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "Role updated.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["role", id] });
    },
  });

  const role = data?.data;
  if (isLoading) return <Skeleton height={400} />;
  if (!role) return <Text>Role not found.</Text>;

  return (
    <Stack>
      <PageHeader
        title={`Edit Role: ${role.name}`}
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Roles", path: "/roles" },
          { label: role.name },
        ]}
      />
      <Card withBorder radius="md" p="lg">
        <RoleForm
          initialValues={role}
          onSubmit={async (v) => {
            await updateMutation.mutateAsync(v);
          }}
          loading={updateMutation.isPending}
        />
      </Card>
    </Stack>
  );
}
