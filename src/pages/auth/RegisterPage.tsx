import {
  TextInput,
  PasswordInput,
  Button,
  Paper,
  Title,
  Text,
  Container,
  Stack,
  Anchor,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { z } from "zod";
import { zodResolver } from "mantine-form-zod-resolver";
import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { notifications } from "@mantine/notifications";
import { authApi } from "../../api/auth";

const schema = z
  .object({
    first_name: z.string().min(2, "First name required"),
    last_name: z.string().min(2, "Last name required"),
    username: z.string().min(3, "Username min 3 characters"),
    email: z.string().email("Valid email required"),
    password: z.string().min(8, "Password min 8 characters"),
    password_confirmation: z.string().min(1, "Please confirm your password"),
    company_name: z.string().optional(),
  })
  .refine((d) => d.password === d.password_confirmation, {
    message: "Passwords do not match",
    path: ["password_confirmation"],
  });

export function RegisterPage() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);

  const form = useForm({
    validate: zodResolver(schema),
    initialValues: {
      first_name: "",
      last_name: "",
      username: "",
      email: "",
      password: "",
      password_confirmation: "", // ✅ Defined here
      company_name: "",
    },
  });

  const handleSubmit = async (values: typeof form.values) => {
    setLoading(true);
    try {
      await authApi.register({
        username: values.username,
        email: values.email,
        password: values.password,
        password_confirmation: values.password_confirmation,
        first_name: values.first_name,
        last_name: values.last_name,
        company_name: values.company_name || undefined,
        role_id: "", // Will be set by backend for new registrations
      });

      notifications.show({
        title: "Account Created!",
        message: "Please check your email to verify your account.",
        color: "green",
      });
      navigate("/login");
    } catch (err: any) {
      notifications.show({
        title: "Error",
        message: err?.response?.data?.message || "Registration failed.",
        color: "red",
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <Container size={440} my={80}>
      <Title ta="center" fw={900}>
        Create Account
      </Title>
      <Text c="dimmed" size="sm" ta="center" mt={5}>
        Already have an account?{" "}
        <Anchor component={Link} to="/login">
          Sign in
        </Anchor>
      </Text>
      <Paper withBorder shadow="md" p={30} mt={30} radius="md">
        <form onSubmit={form.onSubmit(handleSubmit)}>
          <Stack>
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
            <TextInput
              label="Username"
              placeholder="john.doe"
              required
              {...form.getInputProps("username")}
            />
            <TextInput
              label="Email"
              placeholder="john@example.com"
              required
              {...form.getInputProps("email")}
            />
            <TextInput
              label="Company Name (Optional)"
              placeholder="Acme Corp"
              {...form.getInputProps("company_name")}
            />
            <PasswordInput
              label="Password"
              placeholder="Min 8 characters"
              required
              {...form.getInputProps("password")}
            />
            <PasswordInput
              label="Confirm Password"
              placeholder="Re-enter password"
              required
              {...form.getInputProps("password_confirmation")} // ✅ Matches schema
            />
            <Button type="submit" fullWidth loading={loading}>
              Create Account
            </Button>
          </Stack>
        </form>
      </Paper>
    </Container>
  );
}
