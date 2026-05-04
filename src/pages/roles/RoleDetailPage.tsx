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

  const { data: roleData, isLoading, error } = useQuery({
    queryKey: ["role", id],
    queryFn: () => rolesApi.getById(id!),
    enabled: !!id,
  });

  const updateMutation = useMutation({
    mutationFn: async (values: any) => {
      // Only send role_type for non-system roles
      const updateData: any = {
        name: values.name,
        description: values.description,
      };
      if (roleData && !roleData.is_system) {
        updateData.role_type = values.role_type;
      }
      await rolesApi.update(id!, updateData);
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
    onError: (error: any) => {
      const message = error?.response?.data?.message || "Failed to update role";
      notifications.show({
        title: "Error",
        message,
        color: "red",
      });
    },
  });

  const role = roleData;
  if (isLoading) return <Skeleton height={400} />;
  if (error) return <Text c="red">Error loading role: {error?.message}</Text>;
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
