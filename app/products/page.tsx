"use client";

import { useState, useEffect } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ThemeToggle } from "@/components/theme-toggle";
import Link from "next/link";

interface Product {
  id: string;
  name: string;
  price: number;
}

export default function ProductsPage() {
  const [products, setProducts] = useState<Product[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  useEffect(() => {
    // Check if user is authenticated
    const token = localStorage.getItem("token");
    setIsAuthenticated(!!token);
  }, []);

  const fetchProducts = async () => {
    setIsLoading(true);
    setError(null);

    try {
      const token = localStorage.getItem("token");

      if (!token) {
        setError("Please sign in first to view products");
        return;
      }

      const response = await fetch("/api/routes/products", {
        method: "GET",
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
      });

      const data = await response.json();

      if (response.ok) {
        setProducts(data);
      } else {
        setError(data.message || "Failed to fetch products");
      }
    } catch (error) {
      console.error("Products fetch error:", error);
      setError("Network error. Please try again.");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 p-4">
      {/* Theme Toggle - Fixed position */}
      <div className="fixed top-4 right-4 z-10">
        <ThemeToggle />
      </div>

      <div className="max-w-4xl mx-auto space-y-6">
        {/* Header */}
        <Card>
          <CardHeader>
            <CardTitle className="text-center">Products</CardTitle>
            <CardDescription className="text-center">
              View products using the authenticated API
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex gap-4 justify-center">
              <Button
                onClick={fetchProducts}
                disabled={isLoading || !isAuthenticated}
              >
                {isLoading ? "Loading..." : "Fetch Products"}
              </Button>
              <Link href="/">
                <Button variant="outline">Back to Auth</Button>
              </Link>
            </div>

            {!isAuthenticated && (
              <div className="text-center text-red-600 dark:text-red-400">
                ⚠️ You need to sign in first to access products
              </div>
            )}
          </CardContent>
        </Card>

        {/* Products Display */}
        {error && (
          <Card>
            <CardContent className="pt-6">
              <div className="text-center text-red-600 dark:text-red-400">
                ❌ {error}
              </div>
            </CardContent>
          </Card>
        )}

        {products.length > 0 && (
          <Card>
            <CardHeader>
              <CardTitle>Available Products</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                {products.map((product) => (
                  <Card key={product.id} className="border">
                    <CardContent className="pt-6">
                      <div className="space-y-2">
                        <h3 className="font-semibold">{product.name}</h3>
                        <p className="text-lg font-bold text-green-600 dark:text-green-400">
                          ${product.price.toFixed(2)}
                        </p>
                        <p className="text-sm text-gray-600 dark:text-gray-400">
                          ID: {product.id}
                        </p>
                      </div>
                    </CardContent>
                  </Card>
                ))}
              </div>
            </CardContent>
          </Card>
        )}

        {/* Instructions */}
        <Card>
          <CardHeader>
            <CardTitle>How to use</CardTitle>
          </CardHeader>
          <CardContent>
            <ol className="list-decimal list-inside space-y-2 text-sm">
              <li>Go back to the authentication page and sign in</li>
              <li>Return to this page and click &quot;Fetch Products&quot;</li>
              <li>The API will validate your JWT token and return products</li>
              <li>
                If your token is invalid or expired, you&apos;ll see an error
              </li>
            </ol>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
