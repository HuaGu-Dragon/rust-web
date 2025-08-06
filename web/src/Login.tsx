import { useNavigate } from "react-router-dom";
import "./App.css";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "./components/ui/form";
import { useForm, type FieldErrors } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";
import { login } from "./api/auth";
import { setToken } from "./api/http";

function Login() {
  const navigate = useNavigate();

  const formSchema = z.object({
    account: z.string().max(10),
    password: z
      .string()
      .min(6, {
        message: "Password must be at least 6 characters.",
      })
      .max(16),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      account: "",
      password: "",
    },
  });

  async function onSubmit(data: z.infer<typeof formSchema>) {
    try {
      const result = await login({
        account: data.account,
        password: data.password,
      });

      toast.success("Login successful");
      setToken(result);
      navigate("/dashboard");
    } catch (error) {
      toast.error(
        error instanceof Error
          ? error.message.length > 0
            ? error.message
            : "Login failed"
          : "Login failed",
        {
          duration: 2000,
        }
      );
    }
  }

  async function onError(errors: FieldErrors<z.infer<typeof formSchema>>) {
    if (errors.account && errors.password) {
      toast.error("Empty account and password!", {
        duration: 2000,
        icon: "❌",
      });
      console.log(errors);
      return;
    }
    if (errors.account) {
      toast.error("Invalid account!", {
        duration: 2000,
        icon: "❌",
      });
    }
    if (errors.password) {
      toast.error("Invalid password!", {
        duration: 2000,
        icon: "❌",
      });
    }
  }

  return (
    <Card className="w-screen max-w-sm dark">
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit, onError)}>
          <CardHeader>
            <CardTitle>Login to your account</CardTitle>
            <CardDescription>
              Enter your email below to login to your account
            </CardDescription>
            <CardAction>
              <Button variant="link">Sign Up</Button>
            </CardAction>
          </CardHeader>
          <CardContent>
            <div className="flex flex-col gap-6">
              <div className="grid gap-2">
                <Label htmlFor="account">Account</Label>
                <FormField
                  control={form.control}
                  name="account"
                  render={({ field }) => (
                    <FormItem>
                      <FormControl>
                        <Input
                          id="account"
                          {...field}
                          type="text"
                          placeholder="admin"
                          required
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
              <div className="grid gap-2">
                <div className="flex items-center">
                  <Label htmlFor="password">Password</Label>
                  <a
                    href="#"
                    className="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                  >
                    Forgot your password?
                  </a>
                </div>
                <FormField
                  control={form.control}
                  name="password"
                  render={({ field }) => (
                    <FormItem>
                      <FormControl>
                        <Input
                          id="password"
                          {...field}
                          type="password"
                          required
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
            </div>
          </CardContent>
          <CardFooter className="flex-col gap-2 mt-6">
            <Button type="submit" className="w-full">
              Login
            </Button>
            <Button variant="outline" className="w-full">
              Login with Google
            </Button>
          </CardFooter>
        </form>
      </Form>
    </Card>
  );
}

export default Login;
