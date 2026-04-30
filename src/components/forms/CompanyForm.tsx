import {
  TextInput,
  Button,
  Stack,
  Title,
  Text,
  Group,
  Select,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useNavigate, Link } from "react-router-dom";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { companiesApi } from "../../api/companies";

interface CompanyFormProps {
  initialValues?: {
    name?: string;
    email?: string;
    city?: string;
    country?: string;
  };
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
}

export function CompanyForm({
  initialValues,
  onSubmit,
  loading,
}: CompanyFormProps) {
  const form = useForm({
    initialValues: {
      name: initialValues?.name || "",
      email: initialValues?.email || "",
      city: initialValues?.city || "",
      country: initialValues?.country || "",
      is_active: true,
    },
    validate: {
      name: (v) => (!v ? "Company name required" : null),
      email: (v) =>
        v && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v) ? "Invalid email" : null,
    },
  });

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <TextInput
          label="Company Name"
          placeholder="Enter company name"
          required
          {...form.getInputProps("name")}
        />
        <TextInput
          label="Email"
          placeholder="company@example.com"
          {...form.getInputProps("email")}
        />
        <Group grow>
          <TextInput label="City" {...form.getInputProps("city")} />
          <TextInput label="Country" {...form.getInputProps("country")} />
        </Group>
        <Button type="submit" loading={loading}>
          {initialValues ? "Update Company" : "Create Company"}
        </Button>
      </Stack>
    </form>
  );
}
