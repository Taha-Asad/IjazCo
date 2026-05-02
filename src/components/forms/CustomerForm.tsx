import {
  TextInput,
  Textarea,
  NumberInput,
  Switch,
  Button,
  Stack,
  SimpleGrid,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { z } from "zod";
import { zodResolver } from "mantine-form-zod-resolver";

const schema = z.object({
  name: z.string().min(2, "Name required"),
  email: z.string().email("Valid email").optional().or(z.literal("")),
  phone: z.string().optional(),
  address: z.string().optional(),
  city: z.string().optional(),
  country: z.string().optional(),
  credit_limit: z.number().min(0).default(0),
});

interface CustomerFormProps {
  initialValues?: any;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function CustomerForm({
  initialValues,
  onSubmit,
  loading,
}: CustomerFormProps) {
  const form = useForm({
    validate: zodResolver(schema),
    initialValues: {
      name: initialValues?.name || "",
      email: initialValues?.email || "",
      phone: initialValues?.phone || "",
      address: initialValues?.address || "",
      city: initialValues?.city || "",
      country: initialValues?.country || "",
      credit_limit: initialValues?.credit_limit || 0,
      is_active: initialValues?.is_active ?? true,
    },
  });

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <TextInput
          label="Customer Name"
          placeholder="Acme Labs"
          required
          {...form.getInputProps("name")}
        />
        <SimpleGrid cols={2}>
          <TextInput
            label="Email"
            placeholder="customer@email.com"
            {...form.getInputProps("email")}
          />
          <TextInput
            label="Phone"
            placeholder="+1 555 0000"
            {...form.getInputProps("phone")}
          />
        </SimpleGrid>
        <SimpleGrid cols={2}>
          <TextInput label="City" {...form.getInputProps("city")} />
          <TextInput label="Country" {...form.getInputProps("country")} />
        </SimpleGrid>
        <Textarea label="Address" rows={2} {...form.getInputProps("address")} />
        <NumberInput
          label="Credit Limit"
          prefix="$"
          thousandSeparator=","
          min={0}
          {...form.getInputProps("credit_limit")}
        />
        {initialValues && (
          <Switch
            label="Active"
            {...form.getInputProps("is_active", { type: "checkbox" })}
          />
        )}
        <Button type="submit" loading={loading}>
          {initialValues ? "Save Changes" : "Create Customer"}
        </Button>
      </Stack>
    </form>
  );
}
