import { TextInput, Button, Stack, Group } from "@mantine/core";
import { useForm } from "@mantine/form";

interface CustomerFormProps {
  initialValues?: {
    name?: string;
    email?: string;
    phone?: string;
    address?: string;
    city?: string;
    country?: string;
    credit_limit?: number;
  };
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function CustomerForm({
  initialValues,
  onSubmit,
  loading,
}: CustomerFormProps) {
  const form = useForm({
    initialValues: {
      name: initialValues?.name || "",
      email: initialValues?.email || "",
      phone: initialValues?.phone || "",
      address: initialValues?.address || "",
      city: initialValues?.city || "",
      country: initialValues?.country || "",
      credit_limit: initialValues?.credit_limit || 0,
      is_active: true,
    },
    validate: {
      name: (v) => (!v ? "Customer name required" : null),
      email: (v) =>
        v && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v) ? "Invalid email" : null,
    },
  });

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <TextInput
          label="Customer Name"
          placeholder="Enter customer name"
          required
          {...form.getInputProps("name")}
        />
        <Group grow>
          <TextInput label="Email" {...form.getInputProps("email")} />
          <TextInput label="Phone" {...form.getInputProps("phone")} />
        </Group>
        <Group grow>
          <TextInput label="City" {...form.getInputProps("city")} />
          <TextInput label="Country" {...form.getInputProps("country")} />
        </Group>
        <TextInput
          label="Address"
          placeholder="Full address"
          {...form.getInputProps("address")}
        />
        <Button type="submit" loading={loading}>
          {initialValues ? "Update Customer" : "Create Customer"}
        </Button>
      </Stack>
    </form>
  );
}
