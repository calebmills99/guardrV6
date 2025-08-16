import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import Header from "@/components/layout/Header";
import Footer from "@/components/layout/Footer";

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
});

export const metadata: Metadata = {
  title: "Guardr - AI-Powered Dating Safety for LGBTQ+ Community",
  description: "2FA for your heart. AI-enhanced identity verification, real-time risk assessment, and SMS safety alerts for safer online dating. Built for the LGBTQ+ community.",
  keywords: ["dating safety", "LGBTQ+", "AI verification", "online dating protection", "catfish detection"],
  authors: [{ name: "Guardr Team" }],
  creator: "Guardr",
  publisher: "Guardr",
  formatDetection: {
    email: false,
    address: false,
    telephone: false,
  },
  metadataBase: new URL("https://guardr.app"),
  alternates: {
    canonical: "/",
  },
  openGraph: {
    type: "website",
    locale: "en_US",
    url: "https://guardr.app",
    title: "Guardr - AI-Powered Dating Safety for LGBTQ+ Community",
    description: "2FA for your heart. AI-enhanced identity verification and real-time risk assessment for safer online dating.",
    siteName: "Guardr",
    images: [
      {
        url: "/og-image.png",
        width: 1200,
        height: 630,
        alt: "Guardr - Dating Safety Platform",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "Guardr - AI-Powered Dating Safety",
    description: "2FA for your heart. AI-enhanced identity verification for safer online dating.",
    images: ["/og-image.png"],
    creator: "@guardrapp",
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-video-preview": -1,
      "max-image-preview": "large",
      "max-snippet": -1,
    },
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="scroll-smooth">
      <body className={`${inter.variable} font-sans antialiased`}>
        <div className="flex flex-col min-h-screen">
          <Header />
          <main className="flex-grow">
            {children}
          </main>
          <Footer />
        </div>
      </body>
    </html>
  );
}
