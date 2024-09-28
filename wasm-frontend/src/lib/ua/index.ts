export const checkMobileDevice = (): "iPhone" | "Android" | "Other" => {
  const userAgent: string = navigator.userAgent;

  if (/iPhone/i.test(userAgent)) {
    return "iPhone";
  }

  if (/android/i.test(userAgent)) {
    return "Android";
  }

  return "Other";
};
