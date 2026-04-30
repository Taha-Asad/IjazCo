import {
  TextInput,
  Select,
  Button,
  Stack,
  PasswordInput,
  Switch,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { z } from "zod";
import { zodResolver } from "mantine-form-zod-resolver";
import { useQuery } from "@tanstack/react-query";
import { rolesApi } from "../../api/roles";

const createSchema = z.object({
  full_name: z.string().min(2, "Full name required"),
  username: z.string().min(3, "Username min 3 chars"),
  email: z.string().email("Valid email required"),
  password: z.string().min(8, "Password min 8 chars"),
  role_id: z.string().min(1, "Role required"),
});

const editSchema = z.object({
  full_name: z.string().min(2, "Full name required"),
  email: z.string().email("Valid email required"),
  role_id: z.string().min(1, "Role required"),
  is_active: z.boolean(),
});

interface UserFormProps {
  initialValues?: any;
  onSubmit: (values: any) => Promise<void>;
  loading?: boolean;
  mode?: "create" | "edit";
}

export function UserForm({
  initialValues,
  onSubmit,
  loading,
  mode = "create",
}: UserFormProps) {
  const isEdit = mode === "edit";

  const { data: rolesData } = useQuery({
    queryKey: ["roles-list"],
    queryFn: () => rolesApi.list({ per_page: 100 }),
  });

  const form = useForm({
    validate: zodResolver(isEdit ? editSchema : createSchema),
    initialValues: initialValues || {
      full_name: "",
      username: "",
      email: "",
      password: "",
      role_id: "",
      is_active: true,
    },
  });

  const roleOptions =
    rolesData?.data?.map((r) => ({
      value: r.id,
      label: r.name,
    })) || [];

  return (
    <form onSubmit={form.onSubmit(onSubmit)}>
      <Stack>
        <TextInput
          label="Full Name"
          placeholder="John Doe"
          required
          {...form.getInputProps("full_name")}
        />
        {!isEdit && (
          <TextInput
            label="Username"
            placeholder="john.doe"
            required
            {...form.getInputProps("username")}
          />
        )}
        <TextInput
          label="Email"
          placeholder="john@example.com"
          required
          {...form.getInputProps("email")}
        />
        {!isEdit && (
          <PasswordInput
            label="Password"
            placeholder="Min 8 characters"
            required
            {...form.getInputProps("password")}
          />
        )}
        <Select
          label="Role"
          placeholder="Select role"
          data={roleOptions}
          required
          {...form.getInputProps("role_id")}
        />
        {isEdit && (
          <Switch
            label="Active"
            {...form.getInputProps("is_active", { type: "checkbox" })}
          />
        )}
        <Button type="submit" loading={loading}>
          {isEdit ? "Save Changes" : "Create User"}
        </Button>
      </Stack>
    </form>
  );
}
