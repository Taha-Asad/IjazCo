import { useEffect, useMemo, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { requestPasswordReset, verifyEmailToken } from "../services/auth";

const imgIcon = "https://www.figma.com/api/mcp/asset/4d8d1540-03bb-4a0c-a80e-6b0f6ec48701";
const imgIcon1 = "https://www.figma.com/api/mcp/asset/ccf1e078-68aa-4815-9050-fd57f86ee22f";
const imgIcon2 = "https://www.figma.com/api/mcp/asset/01d9660d-f095-4e59-ba59-173cc0c378bf";
const imgIcon3 = "https://www.figma.com/api/mcp/asset/10e2f014-b5b8-4b01-ae79-48754f70e480";

export default function OtpPage() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();

  const [otp, setOtp] = useState("");
  const [countdown, setCountdown] = useState(60);
  const [submitting, setSubmitting] = useState(false);
  const [resending, setResending] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const emailFromQuery = searchParams.get("email") ?? "";

  const email = useMemo(() => {
    // Best-effort: resend OTP typically requires the destination email.
    return emailFromQuery || localStorage.getItem("otp_email") || "";
  }, [emailFromQuery]);

  useEffect(() => {
    setCountdown(60);
    const interval = window.setInterval(() => {
      setCountdown((c) => (c > 0 ? c - 1 : 0));
    }, 1000);
    return () => window.clearInterval(interval);
  }, []);

  const handleOtpChange = (value: string) => {
    setOtp(value.replace(/\D/g, "").slice(0, 6));
    setError(null);
  };

  const handleResend = async () => {
    if (countdown > 0 || resending) return;
    setResending(true);
    setError(null);

    try {
      if (!email) {
        setError("Email is required to resend OTP.");
      } else {
        localStorage.setItem("otp_email", email);
        await requestPasswordReset(email);
      }
    } catch (e: any) {
      setError(e?.message || "Failed to resend OTP.");
    } finally {
      setCountdown(60);
      setResending(false);
    }
  };

  const handleLogin = async () => {
    if (submitting) return;
    setSubmitting(true);
    setError(null);

    try {
      if (otp.length !== 6) {
        setError("Please enter the 6-digit OTP.");
        return;
      }

      // Backend wiring note:
      // - This project does not expose an explicit /auth/otp endpoint.
      // - We map the OTP screen to the existing verify-token endpoint.
      await verifyEmailToken(otp);

      navigate("/login");
    } catch (e: any) {
      setError(e?.message || "OTP verification failed.");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <main className="min-h-screen w-full bg-gradient-to-br from-gray-50 to-white flex items-center justify-center p-4">
      <div className="transform scale-[0.62] origin-top">
        <section
          className="relative w-[706.25px] rounded-[42.375px] shadow-[0px_17.656px_26.484px_-5.297px_rgba(0,0,0,0.05),0px_7.062px_10.594px_-3.531px_rgba(0,0,0,0.02)]"
          style={{ padding: 1.766 }}
        >
          <div
            className="bg-white border-[#f3f4f6] border-[1.766px] border-solid rounded-[42.375px] overflow-hidden"
            role="region"
            aria-label="OTP screen"
          >
            {/* Header */}
            <div className="relative h-[393.293px]">
              <div
                className="absolute bg-[rgba(16,185,129,0.1)] content-stretch flex items-center justify-center left-[294.86px] py-[21.187px] rounded-[28.25px] top-[70.63px] w-[113px]"
                aria-hidden="true"
              >
                <div className="flex items-center justify-center relative shrink-0">
                  <div className="-scale-y-100 flex-none">
                    <div className="relative size-[63.562px]">
                      <img
                        alt=""
                        className="absolute block inset-0 max-w-none size-full"
                        src={imgIcon}
                      />
                    </div>
                  </div>
                </div>
              </div>

              <div className="absolute content-stretch flex flex-col items-center left-[56.5px] right-[56.5px] top-[211.87px]">
                <h1 className="flex flex-col font-['Poppins:Bold',sans-serif] justify-center leading-[0] not-italic relative shrink-0 text-[#111827] text-[42.375px] text-center whitespace-nowrap">
                  Ijaz &amp; Company
                </h1>
              </div>

              <div className="absolute content-stretch flex flex-col gap-[0.441px] items-center left-[56.5px] right-[56.5px] top-[280.73px] text-center">
                <p className="flex flex-col font-['Inter:Medium',sans-serif] font-medium justify-center relative shrink-0 text-[#6b7280] text-[24.719px]">
                  Point of Sale (POS) System
                </p>
                <p className="flex flex-col font-['Inter:Semi_Bold',sans-serif] font-semibold justify-center relative shrink-0 text-[#10b981] text-[21.187px] tracking-[1.0594px] uppercase">
                  Medical Laboratory Equipment
                </p>
              </div>
            </div>

            {/* Form */}
            <div className="relative w-[702.719px] p-[0px]">
              <div className="content-stretch flex flex-col gap-[35.312px] items-start pb-[84.75px] px-[56.5px] relative size-full">
                <div className="content-stretch flex flex-col gap-[10.594px] items-end relative shrink-0 w-full">
                  <div className="content-stretch flex flex-col items-start relative shrink-0 w-[582.656px]">
                    <div className="flex flex-col font-['Inter:Semi_Bold',sans-serif] font-semibold justify-center leading-[0] not-italic relative shrink-0 text-[#374151] text-[21.187px] tracking-[0.5297px] uppercase whitespace-nowrap">
                      Enter OTP
                    </div>
                  </div>

                  <div className="content-stretch flex flex-col items-start relative shrink-0 w-full">
                    <div className="bg-[#f9fafb] border-[#d1d5db] border-[1.766px] border-solid content-stretch flex flex-col items-start overflow-clip pb-[22.953px] pt-[21.188px] px-[72.391px] relative rounded-[14.125px] shrink-0 w-full">
                      <div className="relative shrink-0 w-[444.938px]">
                        <div className="bg-clip-padding border-0 border-[transparent] border-solid content-stretch flex flex-col items-start overflow-clip relative rounded-[inherit] size-full">
                          <input
                            inputMode="numeric"
                            pattern="[0-9]*"
                            aria-label="OTP input"
                            value={otp}
                            onChange={(e) => handleOtpChange(e.target.value)}
                            placeholder="6 Digit Pin"
                            className="w-full bg-transparent border-0 outline-none font-['Inter:Regular',sans-serif] font-normal justify-center leading-[0] not-italic text-[#9ca3af] text-[24.719px] whitespace-nowrap"
                          />
                        </div>
                      </div>

                      {/* Left lock icon */}
                      <div className="absolute bottom-[1.07px] left-0 pl-[21.187px] top-0">
                        <div className="relative size-[35.312px]">
                          <img
                            alt=""
                            className="absolute block inset-0 max-w-none size-full"
                            src={imgIcon1}
                          />
                        </div>
                      </div>

                      {/* Right refresh/submit icon */}
                      <button
                        type="button"
                        onClick={handleLogin}
                        className="absolute bottom-[0.53px] content-stretch flex items-center pr-[21.187px] py-[12.359px] right-0 top-[-0.54px]"
                        aria-label="Verify OTP"
                        disabled={submitting}
                      >
                        <div className="relative size-[35.312px]">
                          <img
                            alt=""
                            className="absolute block inset-0 max-w-none size-full"
                            src={imgIcon2}
                          />
                        </div>
                      </button>
                    </div>

                    {error && (
                      <p className="text-red-600 text-sm mt-3 pl-1">
                        {error}
                      </p>
                    )}
                  </div>
                </div>

                {/* Resend */}
                <div className="content-stretch flex items-center justify-between relative shrink-0 w-full">
                  <button
                    type="button"
                    onClick={handleResend}
                    className="content-stretch flex items-center relative shrink-0"
                    disabled={countdown > 0 || resending}
                  >
                    <span className="bg-[#f3f4f6] border-[#d1d5db] border-[1.766px] border-solid rounded-[14.125px] shrink-0 size-[28.25px] mr-[14.125px] inline-flex items-center justify-center" />
                    <span className="content-stretch flex flex-col items-start pl-[14.125px] relative shrink-0">
                      <span className="flex flex-col font-['Inter:Regular',sans-serif] font-normal justify-center leading-[0] not-italic relative shrink-0 text-[#4b5563] text-[24.719px] whitespace-nowrap">
                        <span className="leading-[35.312px]">Resend OTP </span>
                        <span className="font-['Inter:Bold',sans-serif] font-bold leading-[35.312px] not-italic text-[#10b981] tracking-[1.0594px]">
                          {countdown}s
                        </span>
                      </span>
                    </span>
                  </button>

                  <div className="h-[36px] shrink-0 w-[212px]" aria-hidden="true" />
                </div>

                {/* Login button */}
                <button
                  type="button"
                  onClick={handleLogin}
                  disabled={submitting}
                  className="bg-[#10b981] border-[1.766px] border-[rgba(0,0,0,0)] border-solid content-stretch drop-shadow-[0px_1.766px_1.766px_rgba(0,0,0,0.05)] flex items-center justify-center px-[30.016px] py-[22.953px] relative rounded-[28.25px] shrink-0 w-full"
                >
                  <span className="relative shrink-0 flex flex-col items-center">
                    <span className="flex flex-col font-['Inter:Semi_Bold',sans-serif] font-semibold justify-center leading-[0] not-italic relative shrink-0 text-[24.719px] text-center text-white whitespace-nowrap">
                      Login
                    </span>
                  </span>
                  <span className="relative shrink-0 pl-[14.125px] flex items-center justify-center">
                    <div className="relative size-[31.781px]">
                      <img
                        alt=""
                        className="absolute block inset-0 max-w-none size-full"
                        src={imgIcon3}
                      />
                    </div>
                  </span>
                </button>
              </div>
            </div>

            {/* Footer */}
            <div className="bg-[#f9fafb] border-[#f3f4f6] border-solid border-t-[1.766px] relative shrink-0 w-[702.719px]">
              <div className="bg-clip-padding border-0 border-[transparent] border-solid content-stretch flex items-center justify-between pb-[28.25px] pt-[30.016px] px-[56.5px] relative size-full">
                <div className="relative shrink-0">
                  <div className="bg-clip-padding border-0 border-[transparent] border-solid content-stretch flex flex-col items-start relative size-full">
                    <p className="flex flex-col font-['Consolas:Regular',sans-serif] justify-center leading-[0] not-italic relative shrink-0 text-[#9ca3af] text-[21.187px] whitespace-nowrap">
                      v1.0.4
                    </p>
                  </div>
                </div>

                <div className="bg-white border-[#e5e7eb] border-[1.766px] border-solid drop-shadow-[0px_1.766px_1.766px_rgba(0,0,0,0.05)] relative rounded-[17654.48px] shrink-0">
                  <div className="bg-clip-padding border-0 border-[transparent] border-solid content-stretch flex gap-[10.594px] items-center px-[19.422px] py-[8.828px] relative size-full">
                    <span className="bg-[#059669] rounded-[17654.48px] shrink-0 size-[14.125px]" aria-hidden="true" />
                    <span className="flex flex-col font-['Inter:Medium',sans-serif] font-medium justify-center leading-[0] not-italic relative shrink-0 text-[#6b7280] text-[21.187px] whitespace-nowrap">
                      Online Mode
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
      </div>
    </main>
  );
}

