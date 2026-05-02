import { TextInput, Button, Stack, Textarea } from "@mantine/core";
import { useForm } from "@mantine/form";
import { z } from "zod";
import { zodResolver } from "mantine-form-zod-resolver";

const schema = z.object({
  name: z.string().min(2, "Company name required"),
  email: z.string().email("Valid email").optional().or(z.literal("")),
  phone: z.string().optional(),
  address: z.string().optional(),
  city: z.string().optional(),
  country: z.string().optional(),
});

interface CompanyFormProps {
  initialValues?: any;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function CompanyForm({
  initialValues,
  onSubmit,
  loading,
}: CompanyFormProps) {
  const form = useForm({
    validate: zodResolver(schema),
    initialValues: initialValues || {
      name: "",
      email: "",
      phone: "",
      address: "",
      city: "",
      country: "",
    },
  });

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <TextInput
          label="Company Name"
          placeholder="Acme Corp"
          required
          {...form.getInputProps("name")}
        />
        <TextInput
          label="Email"
          placeholder="info@company.com"
          {...form.getInputProps("email")}
        />
        <TextInput
          label="Phone"
          placeholder="+1 555 0000"
          {...form.getInputProps("phone")}
        />
        <TextInput
          label="City"
          placeholder="New York"
          {...form.getInputProps("city")}
        />
        <TextInput
          label="Country"
          placeholder="USA"
          {...form.getInputProps("country")}
        />
        <Textarea
          label="Address"
          placeholder="Street address"
          rows={3}
          {...form.getInputProps("address")}
        />
        <Button type="submit" loading={loading}>
          {initialValues ? "Save Changes" : "Create Company"}
        </Button>
      </Stack>
    </form>
  );
}
