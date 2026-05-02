import {
  TextInput,
  Textarea,
  Select,
  NumberInput,
  Stack,
  Button,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { z } from "zod";
import { zod4Resolver } from "mantine-form-zod-resolver";

const schema = z.object({
  name: z.string().min(2, "Name required"),
  email: z.string().email("Invalid email").optional().or(z.literal("")),
  phone: z.string().optional(),
  company_name: z.string().optional(),
  status: z.string().optional(),
  source: z.string().optional(),
  estimated_value: z.number().min(0).optional().or(z.nan().transform(() => undefined)),
  description: z.string().optional(),
  expected_close_date: z.string().optional(),
});

interface LeadFormProps {
  initialValues?: any;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function LeadForm({ initialValues, onSubmit, loading }: LeadFormProps) {
  const form = useForm({
    validate: zod4Resolver(schema),
    initialValues: {
      name: initialValues?.name || "",
      email: initialValues?.email || "",
      phone: initialValues?.phone || "",
      company_name: initialValues?.company_name || "",
      status: initialValues?.status || "new",
      source: initialValues?.source || "other",
      estimated_value: initialValues?.estimated_value || "",
      description: initialValues?.description || "",
      expected_close_date: initialValues?.expected_close_date || "",
    },
  });

  const STATUS_OPTIONS = [
    { value: "new", label: "New" },
    { value: "contacted", label: "Contacted" },
    { value: "qualified", label: "Qualified" },
    { value: "proposal", label: "Proposal" },
    { value: "negotiation", label: "Negotiation" },
    { value: "won", label: "Won" },
    { value: "lost", label: "Lost" },
  ];

  const SOURCE_OPTIONS = [
    { value: "website", label: "Website" },
    { value: "referral", label: "Referral" },
    { value: "coldcall", label: "Cold Call" },
    { value: "socialmedia", label: "Social Media" },
    { value: "email", label: "Email" },
    { value: "other", label: "Other" },
  ];

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <TextInput
          label="Name"
          placeholder="John Doe"
          required
          {...form.getInputProps("name")}
        />
        <TextInput
          label="Email"
          placeholder="john@example.com"
          {...form.getInputProps("email")}
        />
        <TextInput
          label="Phone"
          placeholder="+1-555-1234"
          {...form.getInputProps("phone")}
        />
        <TextInput
          label="Company Name"
          placeholder="ABC Corp"
          {...form.getInputProps("company_name")}
        />
        <Select
          label="Status"
          data={STATUS_OPTIONS}
          {...form.getInputProps("status")}
        />
        <Select
          label="Source"
          data={SOURCE_OPTIONS}
          {...form.getInputProps("source")}
        />
        <NumberInput
          label="Estimated Value"
          prefix="$"
          thousandSeparator=","
          decimalScale={2}
          min={0}
          {...form.getInputProps("estimated_value")}
        />
        <Textarea
          label="Description"
          placeholder="Details about the lead..."
          rows={3}
          {...form.getInputProps("description")}
        />
        <Button type="submit" loading={loading}>
          {initialValues ? "Save Changes" : "Create Lead"}
        </Button>
      </Stack>
    </form>
  );
}
