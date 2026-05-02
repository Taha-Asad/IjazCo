import { useState } from "react";
import { useNavigate } from "react-router-dom";
import {
  Stack,
  TextInput,
  Group,
  ActionIcon,
  Tooltip,
  Avatar,
  Badge,
  Menu,
} from "@mantine/core";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import {
  IconSearch,
  IconPlus,
  IconEdit,
  IconTrash,
  IconDots,
  IconLock,
  IconUserCheck,
  IconUserX,
} from "@tabler/icons-react";
import { PageHeader } from "@/components/common/PageHeader";
import { openConfirmModal } from "@/components/common/ConfirmModal";
import { usersApi } from "@/api/users";
import { formatDate } from "@/utils/formatters";
import { useDebounce } from "@/hooks/useDebounce";
import type { User } from "@/types";

const PAGE_SIZE = 20;

export function UsersPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const debouncedSearch = useDebounce(search, 400);

  const { data, isLoading } = useQuery({
    queryKey: ["users", page, debouncedSearch],
    queryFn: () =>
      usersApi.list({
        page: Number(page),
        per_page: Number(PAGE_SIZE),
        ...(debouncedSearch && { search: debouncedSearch }),
      }),
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => usersApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "User deleted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["users"] });
    },
  });

  const statusMutation = useMutation({
    mutationFn: ({ id, is_active }: { id: string; is_active: boolean }) =>
      usersApi.updateStatus(id, { is_active }),
    onSuccess: () => {
      notifications.show({
        title: "Updated",
        message: "User status updated.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["users"] });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Users"
        description="Manage system users and their roles"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Users" }]}
        action={{
          label: "Add User",
          icon: <IconPlus size={16} />,
          onClick: () => navigate("/users/create"),
        }}
      />

      <TextInput
        placeholder="Search users..."
        leftSection={<IconSearch size={16} />}
        value={search}
        onChange={(e) => setSearch(e.currentTarget.value)}
        w={300}
      />

      <DataTable
        records={data?.data || []}
        fetching={isLoading}
        totalRecords={data?.total_items || 0}
        recordsPerPage={PAGE_SIZE}
        page={page}
        onPageChange={setPage}
        minHeight={400}
        columns={[
          {
            accessor: "first_name",
            title: "User",
            render: (user) => (
              <Group gap="sm">
                <Avatar size="sm" radius="xl" color="blue">
                  {user.first_name.charAt(0)}
                </Avatar>
                <div>
                  <div style={{ fontWeight: 500, fontSize: 14 }}>
                    {user.first_name} {user.last_name}
                  </div>
                  <div style={{ color: "gray", fontSize: 12 }}>
                    {user.email}
                  </div>
                </div>
              </Group>
            ),
          },
          { accessor: "username", title: "Username" },
          { accessor: "role_name", title: "Role" },
          {
            accessor: "is_active",
            title: "Status",
            render: (user) => (
              <Badge color={user.is_active ? "green" : "gray"} variant="light">
                {user.is_active ? "Active" : "Inactive"}
              </Badge>
            ),
          },
          {
            accessor: "last_login",
            title: "Last Login",
            render: (user) =>
              user.last_login ? formatDate(user.last_login) : "Never",
          },
          {
            accessor: "actions",
            title: "",
            width: 60,
            render: (user: User) => (
              <Menu shadow="md" width={180}>
                <Menu.Target>
                  <ActionIcon variant="subtle">
                    <IconDots size={16} />
                  </ActionIcon>
                </Menu.Target>
                <Menu.Dropdown>
                  <Menu.Item
                    leftSection={<IconEdit size={14} />}
                    onClick={() => navigate(`/users/${user.id}`)}
                  >
                    Edit
                  </Menu.Item>
                  <Menu.Item
                    leftSection={<IconLock size={14} />}
                    onClick={() => navigate(`/users/${user.id}?tab=password`)}
                  >
                    Change Password
                  </Menu.Item>
                  <Menu.Item
                    leftSection={
                      user.is_active ? (
                        <IconUserX size={14} />
                      ) : (
                        <IconUserCheck size={14} />
                      )
                    }
                    onClick={() =>
                      statusMutation.mutate({
                        id: user.id,
                        is_active: !user.is_active,
                      })
                    }
                  >
                    {user.is_active ? "Deactivate" : "Activate"}
                  </Menu.Item>
                  <Menu.Divider />
                  <Menu.Item
                    color="red"
                    leftSection={<IconTrash size={14} />}
                    onClick={() =>
                      openConfirmModal({
                        title: "Delete User",
                        message: `Delete user "${user.first_name} ${user.last_name}"?`,
                        confirmLabel: "Delete",
                        danger: true,
                        onConfirm: () => deleteMutation.mutate(user.id),
                      })
                    }
                  >
                    Delete
                  </Menu.Item>
                </Menu.Dropdown>
              </Menu>
            ),
          },
        ]}
        highlightOnHover
        withTableBorder
        borderRadius="md"
        striped
      />
    </Stack>
  );
}
