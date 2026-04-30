import {
  TextInput,
  Textarea,
  Button,
  Stack,
  Title,
  Checkbox,
  SimpleGrid,
  Paper,
  Text,
  Group,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useState } from "react";
import { AVAILABLE_RESOURCES, AVAILABLE_ACTIONS } from "../../types/role";

interface RoleFormProps {
  initialValues?: any;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function RoleForm({ initialValues, onSubmit, loading }: RoleFormProps) {
  const [permissions, setPermissions] = useState<Record<string, string[]>>(
    initialValues?.permissions || {},
  );

  const form = useForm({
    initialValues: {
      name: initialValues?.name || "",
      description: initialValues?.description || "",
    },
    validate: {
      name: (v) => (v.trim().length < 2 ? "Name required" : null),
    },
  });

  const togglePermission = (resource: string, action: string) => {
    setPermissions((prev) => {
      const current = prev[resource] || [];
      const updated = current.includes(action)
        ? current.filter((a) => a !== action)
        : [...current, action];
      return { ...prev, [resource]: updated };
    });
  };

  const toggleAll = (resource: string) => {
    setPermissions((prev) => {
      const current = prev[resource] || [];
      const allActions = [...AVAILABLE_ACTIONS];
      const hasAll = allActions.every((a) => current.includes(a));
      return { ...prev, [resource]: hasAll ? [] : allActions };
    });
  };

  const handleSubmit = async (values: typeof form.values) => {
    await onSubmit({ ...values, permissions });
  };

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack>
        <TextInput
          label="Role Name"
          placeholder="e.g. Sales Manager"
          required
          {...form.getInputProps("name")}
        />
        <Textarea
          label="Description"
          placeholder="What does this role do?"
          rows={2}
          {...form.getInputProps("description")}
        />

        <Title order={5} mt="sm">
          Permissions
        </Title>
        {AVAILABLE_RESOURCES.map((resource) => {
          const resourcePerms = permissions[resource] || [];
          const allChecked = AVAILABLE_ACTIONS.every((a) =>
            resourcePerms.includes(a),
          );
          return (
            <Paper key={resource} withBorder p="sm" radius="md">
              <Group justify="space-between" mb="xs">
                <Text fw={600} size="sm" tt="capitalize">
                  {resource}
                </Text>
                <Checkbox
                  label="All"
                  size="xs"
                  checked={allChecked}
                  onChange={() => toggleAll(resource)}
                />
              </Group>
              <SimpleGrid cols={4}>
                {AVAILABLE_ACTIONS.map((action) => (
                  <Checkbox
                    key={action}
                    label={action}
                    size="xs"
                    checked={resourcePerms.includes(action)}
                    onChange={() => togglePermission(resource, action)}
                  />
                ))}
              </SimpleGrid>
            </Paper>
          );
        })}

        <Button type="submit" loading={loading}>
          {initialValues ? "Save Changes" : "Create Role"}
        </Button>
      </Stack>
    </form>
  );
}
