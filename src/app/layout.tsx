import { ThemeProvider } from 'next-themes';
import { AuthProvider } from '@utils/auth';
import { getCurrent } from '@tauri-apps/api/webview';
import '@/app/globals.css';

export interface PageProps {
	children: React.ReactNode;
}

export default function RootLayout({ children }: PageProps) {
	// if (typeof window != 'undefined') getCurrent().setZoom(1.0).catch(console.error);

	return (
		<html className='w-full h-full' suppressHydrationWarning>
			<body className='w-full h-full'>
				<ThemeProvider
					attribute='class'
					defaultTheme='dark'
					enableSystem={ false }
				>
					<AuthProvider>
						{children}
					</AuthProvider>
				</ThemeProvider>
			</body>
		</html>
	);
}
