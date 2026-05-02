import { useState } from "react";
import {
  Stack,
  Group,
  Badge,
  ActionIcon,
  Tooltip,
  Modal,
  Text,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import { IconPlus, IconEdit, IconTrash } from "@tabler/icons-react";
import { useNavigate } from "react-router-dom";
import { PageHeader } from "../../components/common/PageHeader";
import { SearchInput } from "../../components/common/SearchInput";
import { openConfirmModal } from "../../components/common/ConfirmModal";
import { RoleForm } from "../../components/forms/RoleForm";
import { rolesApi } from "../../api/roles";
import { useDebounce } from "../../hooks/useDebounce";
import { useAuthStore } from "../../store/authStore";

const PAGE_SIZE = 20;

export function RolesPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { user } = useAuthStore();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [opened, { open, close }] = useDisclosure(false);
  const debouncedSearch = useDebounce(search, 400);

  // Refetch user data on mount to ensure company_id is loaded
  const { isLoading: userLoading } = useQuery({
    queryKey: ["currentUser"],
    queryFn: async () => {
      const res = await authApi.me();
      if (res.data?.company_id) {
        useAuthStore.getState().setUser(res.data);
      }
      return res.data;
    },
    enabled: !!user && !user?.company_id,
  });

  const { data, isLoading, error } = useQuery({
    queryKey: ["roles", page, debouncedSearch],
    queryFn: () =>
      rolesApi.list({
        page: Number(page),
        per_page: Number(PAGE_SIZE),
        ...(debouncedSearch?.trim() && { search: debouncedSearch }),
      }),
  });

  if (error) {
    notifications.show({
      title: "Error",
      message: (error as any)?.response?.data?.message || "Failed to load roles",
      color: "red",
    });
  }

  const createMutation = useMutation({
    mutationFn: (values: any) => {
      if (!user?.company_id) {
        // Try to get fresh user data
        return authApi.me().then((res) => {
          const freshUser = res.data;
          if (!freshUser?.company_id) {
            throw new Error("Company ID not found. Please log in again.");
          }
          useAuthStore.getState().setUser(freshUser);
          return rolesApi.create({ ...values, company_id: freshUser.company_id });
        });
      }
      return rolesApi.create({ ...values, company_id: user.company_id });
    },
    onSuccess: () => {
      notifications.show({
        title: "Created",
        message: "Role created.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["roles"] });
      close();
    },
    onError: (error: any) => {
      const message = error?.response?.data?.message || "Failed to create role";
      notifications.show({
        title: "Error",
        message,
        color: "red",
      });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (id: string) => rolesApi.delete(id),
    onSuccess: () => {
      notifications.show({
        title: "Deleted",
        message: "Role deleted.",
        color: "green",
      });
      queryClient.invalidateQueries({ queryKey: ["roles"] });
    },
  });

  return (
    <Stack>
      <PageHeader
        title="Roles"
        description="Manage user roles and permissions"
        breadcrumbs={[{ label: "Home", path: "/" }, { label: "Roles" }]}
        action={{
          label: "Create Role",
          icon: <IconPlus size={16} />,
          onClick: open,
        }}
      />
      <SearchInput value={search} onChange={setSearch} w={280} />
      <DataTable
        records={data?.data || data || []}
        fetching={isLoading}
        totalRecords={data?.total_items || data?.length || 0}
        recordsPerPage={PAGE_SIZE}
        page={page}
        onPageChange={setPage}
        columns={[
          { accessor: "name", title: "Role Name" },
          { accessor: "description", title: "Description" },
          {
            accessor: "user_count",
            title: "Users",
            render: (r) => <Badge variant="light">{r.user_count || 0}</Badge>,
          },
          {
            accessor: "actions",
            title: "",
            width: 90,
            render: (r) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="Edit">
                  <ActionIcon
                    variant="subtle"
                    color="blue"
                    onClick={() => navigate(`/roles/${r.id}`)}
                  >
                    <IconEdit size={16} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label="Delete">
                  <ActionIcon
                    variant="subtle"
                    color="red"
                    onClick={() =>
                      openConfirmModal({
                        title: "Delete Role",
                        message: `Delete role "${r.name}"?`,
                        danger: true,
                        onConfirm: () => deleteMutation.mutate(r.id),
                      })
                    }
                  >
                    <IconTrash size={16} />
                  </ActionIcon>
                </Tooltip>
              </Group>
            ),
          },
        ]}
        highlightOnHover
        withTableBorder
        borderRadius="md"
        striped
      />

      <Modal opened={opened} onClose={close} title="Create Role" size="lg">
        <RoleForm
          onSubmit={async (v) => {
            await createMutation.mutateAsync(v);
          }}
          loading={createMutation.isPending}
        />
      </Modal>
    </Stack>
  );
}
