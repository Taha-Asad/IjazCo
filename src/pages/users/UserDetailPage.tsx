import {
  Stack,
  Card,
  Group,
  Title,
  Text,
  Badge,
  Button,
  Tabs,
  Divider,
  Avatar,
  Skeleton,
  PasswordInput,
} from "@mantine/core";
import { useParams, useNavigate, useSearchParams } from "react-router-dom";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { IconEdit, IconArrowLeft } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { UserForm } from "../../components/forms/UserForm";
import { usersApi } from "../../api/users";
import { formatDate, formatDateTime } from "../../utils/formatters";

export function UserDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [params] = useSearchParams();
  const queryClient = useQueryClient();
  const defaultTab = params.get("tab") || "details";

  const { data, isLoading } = useQuery({
    queryKey: ["user", id],
    queryFn: () => usersApi.getById(id!),
    enabled: !!id,
  });

  const updateMutation = useMutation({
    mutationFn: (values: any) => usersApi.update(id!, values),
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "User updated.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["user", id] });
      queryClient.invalidateQueries({ queryKey: ["users"] });
    },
    onError: () => {
      notifications.show({
        title: "Error",
        message: "Update failed.",
        color: "red",
      });
    },
  });

  const user = data?.data;

  if (isLoading) return <Skeleton height={400} />;
  if (!user) return <Text>User not found.</Text>;

  return (
    <Stack>
      <PageHeader
        title={user.full_name}
        description={`@${user.username}`}
        breadcrumbs={[
          { label: "Home", path: "/" },
          { label: "Users", path: "/users" },
          { label: user.full_name },
        ]}
        action={{
          label: "Back",
          icon: <IconArrowLeft size={16} />,
          onClick: () => navigate("/users"),
        }}
      />

      <Card withBorder radius="md" p="lg">
        <Group mb="md">
          <Avatar size={64} radius="xl" color="blue">
            {user.full_name.charAt(0)}
          </Avatar>
          <div>
            <Title order={3}>{user.full_name}</Title>
            <Group gap="xs">
              <Badge color={user.is_active ? "green" : "gray"}>
                {user.is_active ? "Active" : "Inactive"}
              </Badge>
              <Badge variant="outline">{user.role_name}</Badge>
            </Group>
          </div>
        </Group>

        <Divider mb="md" />

        <Tabs defaultValue={defaultTab}>
          <Tabs.List>
            <Tabs.Tab value="details">Details</Tabs.Tab>
            <Tabs.Tab value="edit">Edit</Tabs.Tab>
            <Tabs.Tab value="password">Password</Tabs.Tab>
          </Tabs.List>

          <Tabs.Panel value="details" pt="md">
            <Stack gap="sm">
              <Group justify="space-between">
                <Text c="dimmed" size="sm">
                  Email
                </Text>
                <Text size="sm">{user.email}</Text>
              </Group>
              <Group justify="space-between">
                <Text c="dimmed" size="sm">
                  Username
                </Text>
                <Text size="sm">{user.username}</Text>
              </Group>
              <Group justify="space-between">
                <Text c="dimmed" size="sm">
                  Last Login
                </Text>
                <Text size="sm">
                  {user.last_login ? formatDateTime(user.last_login) : "Never"}
                </Text>
              </Group>
              <Group justify="space-between">
                <Text c="dimmed" size="sm">
                  Created
                </Text>
                <Text size="sm">{formatDate(user.created_at)}</Text>
              </Group>
            </Stack>
          </Tabs.Panel>

          <Tabs.Panel value="edit" pt="md">
            <UserForm
              mode="edit"
              initialValues={user}
              onSubmit={async (v) => {
                await updateMutation.mutateAsync(v);
              }}
              loading={updateMutation.isPending}
            />
          </Tabs.Panel>

          <Tabs.Panel value="password" pt="md">
            <AdminChangePasswordForm userId={id!} />
          </Tabs.Panel>
        </Tabs>
      </Card>
    </Stack>
  );
}

async function AdminChangePasswordForm({ userId }: { userId: string }) {
  const { useForm } = await import("@mantine/form");
  const form = useForm({
    initialValues: { new_password: "", confirm_password: "" },
    validate: {
      new_password: (v) => (v.length < 8 ? "Min 8 characters" : null),
      confirm_password: (v, vals) =>
        v !== vals.new_password ? "Passwords do not match" : null,
    },
  });

  const mutation = useMutation({
    mutationFn: (data: any) => usersApi.changePassword(userId, data),
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "Password updated.",
        color: "green",
      });
      form.reset();
    },
  });

  return (
    <form
      onSubmit={form.onSubmit((v) =>
        mutation.mutate({ new_password: v.new_password }),
      )}
    >
      <Stack>
        <PasswordInput
          label="New Password"
          required
          {...form.getInputProps("new_password")}
        />
        <PasswordInput
          label="Confirm Password"
          required
          {...form.getInputProps("confirm_password")}
        />
        <Button type="submit" loading={mutation.isPending} w={200}>
          Update Password
        </Button>
      </Stack>
    </form>
  );
}
