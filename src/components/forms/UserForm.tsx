import { useEffect, useMemo } from "react";
import {
  TextInput,
  Select,
  Button,
  Stack,
  PasswordInput,
  Switch,
  Group,
} from "@mantine/core";
import { useForm } from "@mantine/form"; // Import resolver from package
import { z } from "zod";
import { useQuery } from "@tanstack/react-query";
import { rolesApi } from "../../api/roles";
import { zodResolver } from "mantine-form-zod-resolver";
import { useAuthStore } from "../../store/authStore";

// --- Single Unified Schema ---
// This is safer than swapping schemas dynamically
const userSchema = z
  .object({
    mode: z.enum(["create", "edit"]),
    first_name: z.string().min(1, "First name required"),
    last_name: z.string().min(1, "Last name required"),
    email: z.string().email("Valid email required"),
    role_id: z.string().min(1, "Role required"),
    is_active: z.boolean(),
    // Conditional validation logic
    username: z
      .string()
      .optional()
      .refine((val) => {
        // If not in edit mode, username must be at least 3 chars
        return true; // Logic handled inside superRefine for better error mapping
      }),
    password: z.string().optional(),
  })
  .superRefine((data, ctx) => {
    if (data.mode === "create") {
      if (!data.username || data.username.length < 3) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "Username min 3 chars",
          path: ["username"],
        });
      }
      if (!data.password || data.password.length < 8) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "Password min 8 chars",
          path: ["password"],
        });
      }
    }
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

  const { user } = useAuthStore();

  const { data: rolesData } = useQuery({
    queryKey: ["roles-list", user?.company_id],
    queryFn: async () => {
      const res = await rolesApi.list({
        per_page: 100,
        company_id: user?.company_id,
      });
      return res;
    },
    enabled: !!user?.company_id,
  });

  const form = useForm({
    validate: zodResolver(userSchema),
    initialValues: {
      mode: mode,
      first_name: "",
      last_name: "",
      username: "",
      email: "",
      password: "",
      role_id: "",
      is_active: true,
    },
  });

  // IMPORTANT: Sync external data with the form
  useEffect(() => {
    if (initialValues) {
      form.setValues({
        mode: mode,
        first_name: initialValues.first_name ?? "",
        last_name: initialValues.last_name ?? "",
        username: initialValues.username ?? "",
        email: initialValues.email ?? "",
        password: "", // Always empty on edit for security
        role_id: initialValues.role_id ? String(initialValues.role_id) : "",
        is_active: initialValues.is_active ?? true,
      });
      form.resetDirty();
    }
  }, [initialValues, mode]);

  // Handle the "data" property correctly based on your rolesApi
  const roleOptions = useMemo(() => {
    // If rolesApi returns PaginatedResponse, access the data array
    const list = Array.isArray(rolesData) ? rolesData : rolesData?.data;
    return (list || []).map((r: any) => ({
      value: String(r.id),
      label: r.name,
    }));
  }, [rolesData]);

  return (
    <form onSubmit={form.onSubmit((values) => onSubmit(values))}>
      <Stack>
        <Group grow>
          <TextInput
            label="First Name"
            placeholder="John"
            required
            {...form.getInputProps("first_name")}
          />
          <TextInput
            label="Last Name"
            placeholder="Doe"
            required
            {...form.getInputProps("last_name")}
          />
        </Group>

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
          searchable
          required
          {...form.getInputProps("role_id")}
        />

        {isEdit && (
          <Switch
            label="Active status"
            {...form.getInputProps("is_active", { type: "checkbox" })}
          />
        )}

        <Button type="submit" loading={loading} mt="md">
          {isEdit ? "Save Changes" : "Create User"}
        </Button>
      </Stack>
    </form>
  );
}
