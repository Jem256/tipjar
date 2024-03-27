import Navbar from '@/components/Navbar';
import './globals.css';
import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import Footer from '@/components/Footer';
import { Analytics } from '@vercel/analytics/react';
import PlausibleProvider from 'next-plausible';
import StoreProvider from './services/storeProvider';

const inter = Inter({ subsets: ['latin'] });

let title = 'TipJar - Lightning-Powered Tipping Platform';
let description = 'Generate your Payment Link in seconds';

export const metadata: Metadata = {
    title,
    description,
    icons: {
        icon: '/favicon.ico',
    },
    openGraph: {
        title,
        description,
        locale: 'en_US',
        type: 'website',
    },
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <StoreProvider>
            <html lang='en'>
                <head>
                    <PlausibleProvider domain='/' />
                </head>
                <body
                    className={`flex flex-col min-h-screen ${inter.className}`}
                >
                    <Navbar />
                    <main className='flex-1 flex justify-center items-center'>
                        {children}
                    </main>
                    <Analytics />
                    <Footer />
                </body>
            </html>
        </StoreProvider>
    );
}
