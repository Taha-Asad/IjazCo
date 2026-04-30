import {
  Menu,
  Avatar,
  Group,
  Text,
  UnstyledButton,
  Divider,
  Badge,
} from "@mantine/core";
import {
  IconUser,
  IconSettings,
  IconLogout,
  IconBell,
  IconChevronDown,
} from "@tabler/icons-react";
import { useNavigate } from "react-router-dom";
import { notifications } from "@mantine/notifications";
import { useAuthStore } from "../../store/authStore";
import { useNotificationStore } from "../../store/notificationStore";
import { authApi } from "../../api/auth";

export function UserMenu() {
  const navigate = useNavigate();
  const { user, logout } = useAuthStore();
  const { unreadCount } = useNotificationStore();

  const handleLogout = async () => {
    try {
      await authApi.logout();
    } finally {
      logout();
      navigate("/login");
      notifications.show({
        title: "Signed out",
        message: "You have been successfully signed out.",
        color: "blue",
      });
    }
  };

  const initials =
    user?.full_name
      ?.split(" ")
      .map((n: string) => n[0])
      .join("")
      .toUpperCase()
      .slice(0, 2) || "U";

  return (
    <Menu shadow="md" width={220} position="bottom-end">
      <Menu.Target>
        <UnstyledButton>
          <Group gap="xs">
            <Avatar size="sm" radius="xl" color="erp-green">
              {initials}
            </Avatar>
            <div style={{ lineHeight: 1.2 }}>
              <Text size="sm" fw={600} lineClamp={1}>
                {user?.full_name}
              </Text>
              <Text size="xs" c="dimmed" lineClamp={1}>
                {user?.role_name}
              </Text>
            </div>
            {unreadCount > 0 && (
              <Badge size="xs" color="red" circle>
                {unreadCount}
              </Badge>
            )}
            <IconChevronDown size={14} />
          </Group>
        </UnstyledButton>
      </Menu.Target>

      <Menu.Dropdown>
        <Menu.Label>
          <Text size="xs" c="dimmed">
            {user?.email}
          </Text>
        </Menu.Label>
        <Menu.Item
          leftSection={<IconUser size={14} />}
          onClick={() => navigate("/settings")}
        >
          Profile
        </Menu.Item>
        <Menu.Item
          leftSection={<IconBell size={14} />}
          rightSection={
            unreadCount > 0 ? (
              <Badge size="xs" color="red">
                {unreadCount}
              </Badge>
            ) : null
          }
        >
          Notifications
        </Menu.Item>
        <Menu.Item
          leftSection={<IconSettings size={14} />}
          onClick={() => navigate("/settings")}
        >
          Settings
        </Menu.Item>
        <Divider />
        <Menu.Item
          color="red"
          leftSection={<IconLogout size={14} />}
          onClick={handleLogout}
        >
          Sign Out
        </Menu.Item>
      </Menu.Dropdown>
    </Menu>
  );
}
