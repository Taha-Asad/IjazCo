import { useState } from "react";
import {
  Stack,
  Card,
  Title,
  Text,
  TextInput,
  PasswordInput,
  Button,
  Divider,
  Group,
  Avatar,
  Switch,
  Select,
  Tabs,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import { useMantineColorScheme } from "@mantine/core";
import { IconUser, IconLock, IconPalette } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { useAuthStore } from "../../store/authStore";
import { authApi } from "../../api/auth";

export function SettingsPage() {
  const { user } = useAuthStore();
  const { colorScheme, toggleColorScheme } = useMantineColorScheme();
  const [passwordLoading, setPasswordLoading] = useState(false);

  const passwordForm = useForm({
    initialValues: {
      current_password: "",
      new_password: "",
      confirm_password: "",
    },
    validate: {
      new_password: (v) => (v.length < 8 ? "Min 8 characters" : null),
      confirm_password: (v, values) =>
        v !== values.new_password ? "Passwords do not match" : null,
    },
  });

  const handleChangePassword = async (values: typeof passwordForm.values) => {
    setPasswordLoading(true);
    try {
      await authApi.changePassword({
        current_password: values.current_password,
        new_password: values.new_password,
      });
      notifications.show({
        title: "Success",
        message: "Password changed successfully.",
        color: "green",
      });
      passwordForm.reset();
    } catch (err: any) {
      notifications.show({
        title: "Error",
        message: err?.response?.data?.message || "Failed to change password.",
        color: "red",
      });
    } finally {
      setPasswordLoading(false);
    }
  };

  return (
    <Stack>
      <PageHeader
        title="Settings"
        description="Manage your account and preferences"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Settings" }]}
      />

      <Tabs defaultValue="profile">
        <Tabs.List>
          <Tabs.Tab value="profile" leftSection={<IconUser size={16} />}>
            Profile
          </Tabs.Tab>
          <Tabs.Tab value="security" leftSection={<IconLock size={16} />}>
            Security
          </Tabs.Tab>
          <Tabs.Tab value="appearance" leftSection={<IconPalette size={16} />}>
            Appearance
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="profile" pt="md">
          <Card withBorder radius="md" p="lg">
            <Group mb="lg">
              <Avatar size={64} radius="xl" color="blue">
                {user?.full_name?.charAt(0)}
              </Avatar>
              <div>
                <Title order={4}>{user?.full_name}</Title>
                <Text c="dimmed" size="sm">
                  {user?.email}
                </Text>
                <Text c="dimmed" size="xs">
                  {user?.role_name}
                </Text>
              </div>
            </Group>
            <Divider mb="md" />
            <Stack gap="sm">
              <TextInput
                label="Full Name"
                value={user?.full_name || ""}
                readOnly
              />
              <TextInput
                label="Username"
                value={user?.username || ""}
                readOnly
              />
              <TextInput label="Email" value={user?.email || ""} readOnly />
              <TextInput label="Role" value={user?.role_name || ""} readOnly />
            </Stack>
          </Card>
        </Tabs.Panel>

        <Tabs.Panel value="security" pt="md">
          <Card withBorder radius="md" p="lg">
            <Title order={4} mb="md">
              Change Password
            </Title>
            <form onSubmit={passwordForm.onSubmit(handleChangePassword)}>
              <Stack>
                <PasswordInput
                  label="Current Password"
                  required
                  {...passwordForm.getInputProps("current_password")}
                />
                <PasswordInput
                  label="New Password"
                  required
                  {...passwordForm.getInputProps("new_password")}
                />
                <PasswordInput
                  label="Confirm New Password"
                  required
                  {...passwordForm.getInputProps("confirm_password")}
                />
                <Button type="submit" loading={passwordLoading} w={200}>
                  Update Password
                </Button>
              </Stack>
            </form>
          </Card>
        </Tabs.Panel>

        <Tabs.Panel value="appearance" pt="md">
          <Card withBorder radius="md" p="lg">
            <Title order={4} mb="md">
              Appearance
            </Title>
            <Stack>
              <Group justify="space-between">
                <div>
                  <Text fw={500}>Dark Mode</Text>
                  <Text size="sm" c="dimmed">
                    Switch between light and dark theme
                  </Text>
                </div>
                <Switch
                  checked={colorScheme === "dark"}
                  onChange={toggleColorScheme}
                  size="lg"
                />
              </Group>
            </Stack>
          </Card>
        </Tabs.Panel>
      </Tabs>
    </Stack>
  );
}
