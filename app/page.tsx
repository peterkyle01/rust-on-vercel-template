"use client";

import { useState } from "react";
import Link from "next/link";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { ThemeToggle } from "@/components/theme-toggle";
import { AuthResponse, User } from "@/types/models";

export default function Home() {
  const [activeTab, setActiveTab] = useState("signin");
  const [isLoading, setIsLoading] = useState(false);
  const [tokenCopied, setTokenCopied] = useState(false);
  const [authStatus, setAuthStatus] = useState<{
    isLoggedIn: boolean;
    user?: User;
    token?: string;
    error?: string;
  }>({
    isLoggedIn: false,
  });

  // Form states
  const [signinForm, setSigninForm] = useState({ email: "", password: "" });
  const [signupForm, setSignupForm] = useState({
    email: "",
    username: "",
    password: "",
  });

  const handleSignin = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);

    try {
      const response = await fetch("/api/auth/signin", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(signinForm),
      });

      const data = await response.json();

      if (response.ok) {
        const authData: AuthResponse = data;
        setAuthStatus({
          isLoggedIn: true,
          user: authData.user,
          token: authData.token,
        });
        // Store token in localStorage for future requests
        localStorage.setItem("token", authData.token);
      } else {
        setAuthStatus({
          isLoggedIn: false,
          error: data.message || "Signin failed",
        });
      }
    } catch (error) {
      console.error("Signin error:", error);
      setAuthStatus({
        isLoggedIn: false,
        error: "Network error. Please try again.",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleSignup = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);

    try {
      const response = await fetch("/api/auth/signup", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(signupForm),
      });

      const data = await response.json();

      if (response.ok) {
        const authData: AuthResponse = data;
        setAuthStatus({
          isLoggedIn: true,
          user: authData.user,
          token: authData.token,
        });
        // Store token in localStorage for future requests
        localStorage.setItem("token", authData.token);
      } else {
        setAuthStatus({
          isLoggedIn: false,
          error: data.message || "Signup failed",
        });
      }
    } catch (error) {
      console.error("Signup error:", error);
      setAuthStatus({
        isLoggedIn: false,
        error: "Network error. Please try again.",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleSignout = () => {
    setAuthStatus({ isLoggedIn: false });
    localStorage.removeItem("token");
    setSigninForm({ email: "", password: "" });
    setSignupForm({ email: "", username: "", password: "" });
    setTokenCopied(false);
  };

  const copyTokenToClipboard = async () => {
    if (authStatus.token) {
      try {
        await navigator.clipboard.writeText(authStatus.token);
        setTokenCopied(true);
        setTimeout(() => setTokenCopied(false), 2000); // Reset after 2 seconds
      } catch (err) {
        console.error("Failed to copy token:", err);
      }
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 flex items-center justify-center p-4">
      {/* Theme Toggle - Fixed position */}
      <div className="fixed top-4 right-4 z-10">
        <ThemeToggle />
      </div>

      <div className="w-full max-w-md space-y-6">
        {/* Authentication Tabs */}
        <Card>
          <CardHeader>
            <CardTitle className="text-center">Authentication</CardTitle>
            <CardDescription className="text-center">
              Sign in to your account or create a new one
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Tabs
              value={activeTab}
              onValueChange={setActiveTab}
              className="w-full"
            >
              <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="signin">Sign In</TabsTrigger>
                <TabsTrigger value="signup">Sign Up</TabsTrigger>
              </TabsList>

              <TabsContent value="signin" className="space-y-4">
                <form onSubmit={handleSignin} className="space-y-4">
                  <div>
                    <Input
                      type="email"
                      placeholder="Email"
                      value={signinForm.email}
                      onChange={(e) =>
                        setSigninForm({ ...signinForm, email: e.target.value })
                      }
                      required
                    />
                  </div>
                  <div>
                    <Input
                      type="password"
                      placeholder="Password"
                      value={signinForm.password}
                      onChange={(e) =>
                        setSigninForm({
                          ...signinForm,
                          password: e.target.value,
                        })
                      }
                      required
                    />
                  </div>
                  <Button type="submit" className="w-full" disabled={isLoading}>
                    {isLoading ? "Signing in..." : "Sign In"}
                  </Button>
                </form>
              </TabsContent>

              <TabsContent value="signup" className="space-y-4">
                <form onSubmit={handleSignup} className="space-y-4">
                  <div>
                    <Input
                      type="email"
                      placeholder="Email"
                      value={signupForm.email}
                      onChange={(e) =>
                        setSignupForm({ ...signupForm, email: e.target.value })
                      }
                      required
                    />
                  </div>
                  <div>
                    <Input
                      type="text"
                      placeholder="Username"
                      value={signupForm.username}
                      onChange={(e) =>
                        setSignupForm({
                          ...signupForm,
                          username: e.target.value,
                        })
                      }
                      required
                    />
                  </div>
                  <div>
                    <Input
                      type="password"
                      placeholder="Password (min 6 characters)"
                      value={signupForm.password}
                      onChange={(e) =>
                        setSignupForm({
                          ...signupForm,
                          password: e.target.value,
                        })
                      }
                      required
                      minLength={6}
                    />
                  </div>
                  <Button type="submit" className="w-full" disabled={isLoading}>
                    {isLoading ? "Creating account..." : "Sign Up"}
                  </Button>
                </form>
              </TabsContent>
            </Tabs>
          </CardContent>
        </Card>

        {/* Status Card */}
        <Card>
          <CardHeader>
            <CardTitle className="text-center">Status</CardTitle>
          </CardHeader>
          <CardContent>
            {authStatus.isLoggedIn ? (
              <div className="space-y-4">
                <div className="text-center">
                  <div className="text-green-600 dark:text-green-400 font-medium mb-2">
                    ✅ Successfully signed in!
                  </div>
                  <div className="space-y-2 text-sm">
                    <p>
                      <strong>Username:</strong> {authStatus.user?.username}
                    </p>
                    <p>
                      <strong>Email:</strong> {authStatus.user?.email}
                    </p>
                  </div>
                  <div className="mt-4 p-3 bg-gray-100 dark:bg-gray-800 rounded border">
                    <div className="flex items-center justify-between mb-2">
                      <p className="text-sm font-medium">
                        <strong>JWT Token:</strong>
                      </p>
                      <Button
                        size="sm"
                        variant="ghost"
                        onClick={copyTokenToClipboard}
                        className="h-6 px-2 text-xs"
                      >
                        {tokenCopied ? "Copied!" : "Copy"}
                      </Button>
                    </div>
                    <div
                      className="break-all font-mono text-xs p-2 bg-white dark:bg-gray-900 rounded border cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
                      onClick={copyTokenToClipboard}
                      title="Click to copy token"
                    >
                      {authStatus.token}
                    </div>
                  </div>
                </div>
                <Button
                  onClick={handleSignout}
                  variant="outline"
                  className="w-full"
                >
                  Sign Out
                </Button>
                <Link href="/products">
                  <Button variant="secondary" className="w-full">
                    View Products
                  </Button>
                </Link>
              </div>
            ) : (
              <div className="text-center">
                {authStatus.error ? (
                  <div className="text-red-600 dark:text-red-400">
                    ❌ {authStatus.error}
                  </div>
                ) : (
                  <div className="text-gray-600 dark:text-gray-400">
                    🔒 You are not logged in
                  </div>
                )}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
