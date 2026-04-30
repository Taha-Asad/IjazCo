import {
  TextInput,
  Textarea,
  NumberInput,
  Button,
  Stack,
  SimpleGrid,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { z } from "zod";
import { zodResolver } from "mantine-form-zod-resolver";

const schema = z.object({
  name: z.string().min(2, "Supplier name required"),
  email: z.string().email("Valid email").optional().or(z.literal("")),
  phone: z.string().optional(),
  contact_person: z.string().optional(),
  payment_terms: z.number().min(0).optional(),
  city: z.string().optional(),
  country: z.string().optional(),
  address: z.string().optional(),
});

interface SupplierFormProps {
  initialValues?: any;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function SupplierForm({
  initialValues,
  onSubmit,
  loading,
}: SupplierFormProps) {
  const form = useForm({
    validate: zodResolver(schema),
    initialValues: {
      name: initialValues?.name || "",
      email: initialValues?.email || "",
      phone: initialValues?.phone || "",
      contact_person: initialValues?.contact_person || "",
      payment_terms: initialValues?.payment_terms || 30,
      city: initialValues?.city || "",
      country: initialValues?.country || "",
      address: initialValues?.address || "",
    },
  });

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <TextInput
          label="Supplier Name"
          placeholder="Scientific Supplies Co."
          required
          {...form.getInputProps("name")}
        />
        <TextInput
          label="Contact Person"
          placeholder="Jane Smith"
          {...form.getInputProps("contact_person")}
        />
        <SimpleGrid cols={2}>
          <TextInput
            label="Email"
            placeholder="supplier@email.com"
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
          label="Payment Terms (days)"
          min={0}
          max={365}
          {...form.getInputProps("payment_terms")}
        />
        <Button type="submit" loading={loading}>
          {initialValues ? "Save Changes" : "Create Supplier"}
        </Button>
      </Stack>
    </form>
  );
}
