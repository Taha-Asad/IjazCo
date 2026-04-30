import { useState } from "react";
import { Stack, Group, Badge, ActionIcon, Tooltip, Modal, Text, Title, Divider } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { DataTable } from "mantine-datatable";
import { notifications } from "@mantine/notifications";
import { IconEdit, IconTrash, IconShield } from "@tabler/icons-react";
import { PageHeader } from "../../components/common/PageHeader";
import { RoleForm } from "../../components/forms/RoleForm";
import { rolesApi, type Role } from "../../api/roles";

const PAGE_SIZE = 20;

export function RoleDetailPage() {
  const queryClient = useQueryClient();
  const [page, setPage] = useState(1);
  const [selected, setSelected] = useState<Role | null>(null);
  const [editOpened, { open: openEdit, close: closeEdit }] = useDisclosure(false);

  const { data, isLoading } = useQuery({
    queryKey: ["roles", page],
    queryFn: () => rolesApi.list({ page, per_page: PAGE_SIZE }),
  });

  const deleteMutation = useMutation({
    mutationFn: rolesApi.delete,
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
      />
      <DataTable
        records={data?.data || []}
        fetching={isLoading}
        totalRecords={data?.total_items || 0}
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
            width: 100,
            render: (r: Role) => (
              <Group gap="xs" justify="flex-end">
                <Tooltip label="Edit">
                  <ActionIcon
                    variant="subtle"
                    color="blue"
                    onClick={() => {
                      setSelected(r);
                      openEdit();
                    }}
                  >
                    <IconEdit size={16} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label="Delete">
                  <ActionIcon
                    variant="subtle"
                    color="red"
                    onClick={() => deleteMutation.mutateAsync(r.id)}
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
      <Modal opened={editOpened} onClose={closeEdit} title="Edit Role">
        {selected && (
          <RoleForm
            initialValues={selected}
            onSubmit={async () => {
              closeEdit();
            }}
          />
        )}
      </Modal>
    </Stack>
  );
}
