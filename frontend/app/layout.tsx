import type {Metadata} from "next";
import {Inter} from "next/font/google";
import {ThemeProvider} from '@/context/ThemeContext';
import {GoogleAnalytics} from '@next/third-parties/google'
import "./globals.css";
import React from "react";
import {StreamerProvider} from "@/context/StreamersContext";
import GoogleAdsense from "@/components/func/AdSense";

const inter = Inter({subsets: ["latin"]});

export const metadata: Metadata = {
    title: "卢的弹幕监控",
    description: "高强度监视中-卢的弹幕监控",
    referrer: "no-referrer",
};

export default function RootLayout({
                                       children,
                                   }: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="zh">
        <body className={inter.className}>
        <StreamerProvider>
        <ThemeProvider>
                    {children}
        </ThemeProvider>
        </StreamerProvider>
        </body>
        <GoogleAnalytics gaId="G-F6SKTB30TH"/>
        <GoogleAdsense pId={"9580858582528088"}/>
        </html>
    );
}
