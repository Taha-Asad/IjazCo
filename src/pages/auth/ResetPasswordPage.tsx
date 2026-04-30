import {
  PasswordInput,
  Button,
  Paper,
  Title,
  Text,
  Container,
  Stack,
  Alert,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { z } from "zod";
import { zodResolver } from "mantine-form-zod-resolver";
import { useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { notifications } from "@mantine/notifications";
import { IconCheck } from "@tabler/icons-react";
import { authApi } from "../../api/auth";

const schema = z
  .object({
    new_password: z.string().min(8, "Min 8 characters"),
    confirm_password: z.string(),
  })
  .refine((d) => d.new_password === d.confirm_password, {
    message: "Passwords do not match",
    path: ["confirm_password"],
  });

export function ResetPasswordPage() {
  const navigate = useNavigate();
  const [params] = useSearchParams();
  const token = params.get("token") || "";
  const [loading, setLoading] = useState(false);
  const [done, setDone] = useState(false);

  const form = useForm({
    validate: zodResolver(schema),
    initialValues: { new_password: "", confirm_password: "" },
  });

  const handleSubmit = async (values: typeof form.values) => {
    if (!token) return;
    setLoading(true);
    try {
      await authApi.resetPassword({ token, new_password: values.new_password });
      setDone(true);
      setTimeout(() => navigate("/login"), 3000);
    } catch (err: any) {
      notifications.show({
        title: "Error",
        message:
          err?.response?.data?.message ||
          "Reset failed. Link may have expired.",
        color: "red",
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <Container size={440} my={80}>
      <Title ta="center" fw={900}>
        Reset Password
      </Title>
      <Paper withBorder shadow="md" p={30} mt={30} radius="md">
        {done ? (
          <Alert icon={<IconCheck />} color="green">
            Password reset successfully. Redirecting to login...
          </Alert>
        ) : !token ? (
          <Text c="red" ta="center">
            Invalid or missing reset token.
          </Text>
        ) : (
          <form onSubmit={form.onSubmit(handleSubmit)}>
            <Stack>
              <PasswordInput
                label="New Password"
                required
                {...form.getInputProps("new_password")}
              />
              <PasswordInput
                label="Confirm Password"
                required
                {...form.getInputProps("confirm_password")}
              />
              <Button type="submit" fullWidth loading={loading}>
                Reset Password
              </Button>
            </Stack>
          </form>
        )}
      </Paper>
    </Container>
  );
}
