import {
  Group,
  Text,
  Avatar,
  Menu,
  ActionIcon,
  useMantineColorScheme,
} from "@mantine/core";
import {
  IconSun,
  IconMoon,
  IconUser,
  IconLogout,
  IconSettings,
  IconChevronDown,
} from "@tabler/icons-react";
import { useNavigate } from "react-router-dom";
import { useAuthStore } from "@/store/authStore";
import { authApi } from "@/api/auth";
import { notifications } from "@mantine/notifications";

export function Header() {
  const navigate = useNavigate();
  const { user, logout } = useAuthStore();
  const { colorScheme, toggleColorScheme } = useMantineColorScheme();

  const handleLogout = async () => {
    try {
      await authApi.logout();
    } finally {
      logout();
      navigate("/login");
      notifications.show({
        title: "Logged Out",
        message: "You have been successfully logged out.",
        color: "blue",
      });
    }
  };

  return (
    <Group justify="space-between" w="100%">
      <Text fw={700} size="lg" c="erp-green">
        🏭 TauriERP
      </Text>

      <Group>
        <ActionIcon
          variant="subtle"
          onClick={toggleColorScheme}
          size="lg"
          aria-label="Toggle color scheme"
        >
          {colorScheme === "dark" ? (
            <IconSun size={18} />
          ) : (
            <IconMoon size={18} />
          )}
        </ActionIcon>

        <Menu shadow="md" width={200}>
          <Menu.Target>
            <Group style={{ cursor: "pointer" }} gap="xs">
              <Avatar size="sm" radius="xl" color="erp-green">
                {user?.first_name?.charAt(0).toUpperCase()}
              </Avatar>
              <div>
                <Text size="sm" fw={500} lineClamp={1}>
                  {user?.first_name} {user?.last_name}
                </Text>
                <Text size="xs" c="dimmed" lineClamp={1}>
                  {user?.role_name}
                </Text>
              </div>
              <IconChevronDown size={14} />
            </Group>
          </Menu.Target>

          <Menu.Dropdown>
            <Menu.Label>Account</Menu.Label>
            <Menu.Item
              leftSection={<IconUser size={14} />}
              onClick={() => navigate("/settings")}
            >
              Profile
            </Menu.Item>
            <Menu.Item
              leftSection={<IconSettings size={14} />}
              onClick={() => navigate("/settings")}
            >
              Settings
            </Menu.Item>
            <Menu.Divider />
            <Menu.Item
              color="red"
              leftSection={<IconLogout size={14} />}
              onClick={handleLogout}
            >
              Logout
            </Menu.Item>
          </Menu.Dropdown>
        </Menu>
      </Group>
    </Group>
  );
}
