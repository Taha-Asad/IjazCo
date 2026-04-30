import { Stack, Card } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { PageHeader } from "../../components/common/PageHeader";
import { UserForm } from "../../components/forms/UserForm";
import { usersApi } from "../../api/users";
import { useAuthStore } from "../../store/authStore";

export function CreateUserPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { user } = useAuthStore();

  const mutation = useMutation({
    mutationFn: (data: any) =>
      usersApi.create({ ...data, company_id: user?.company_id }),
    onSuccess: (res) => {
      notifications.show({
        title: "Created",
        message: "User created.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["users"] });
      navigate(`/users/${res.data.id}`);
    },
    onError: (err: any) => {
      notifications.show({
        title: "Error",
        message: err?.response?.data?.message || "Failed to create user.",
        color: "red",
      });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Create User"
        description="Add a new user to the system"
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Users", path: "/users" },
          { label: "Create" },
        ]}
      />
      <Card withBorder radius="md" p="lg" maw={600}>
        <UserForm
          onSubmit={async (v) => {
            await mutation.mutateAsync(v);
          }}
          loading={mutation.isPending}
        />
      </Card>
    </Stack>
  );
}
