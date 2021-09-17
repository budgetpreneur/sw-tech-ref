using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using PubnubApi;

namespace LoginApp.Data
{
    public class PubnubApplicationState
    {
        public event Action OnChange;

        static PNConfiguration _config;

        public class PlatformPubnubLog : IPubnubLog
        {
            public PlatformPubnubLog()
            {
                // Get folder path may vary based on environment
                string folder = System.IO.Directory.GetCurrentDirectory(); //For console
                //string folder = System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments); // For iOS
                System.Diagnostics.Debug.WriteLine(folder);
                //string logFilePath = System.IO.Path.Combine(folder, "pubnubmessaging.log");
                //Trace.Listeners.Add(new TextWriterTraceListener(logFilePath));
            }

            void IPubnubLog.WriteToLog(string log)
            {
                System.Diagnostics.Debug.WriteLine(log);
                //Trace.WriteLine(log);
                //Trace.Flush();
            }
        }
        static PubnubApplicationState()
        {
            _config = new PNConfiguration()
            {
                SubscribeKey = "demo",
                PublishKey = "demo",
                SecretKey = "demo",
                CipherKey = "hackathon-master-key",
                Uuid = "admin@uuid.com",
                ReconnectionPolicy = PNReconnectionPolicy.LINEAR,
                //LogVerbosity = PNLogVerbosity.BODY,
                //PubnubLog = new PlatformPubnubLog()
                LogVerbosity = PNLogVerbosity.NONE,
            };
        }
        public PubnubApplicationState()
        {
        }
        public PubnubApplicationState(PNConfiguration config)
        {
            _config = config;
        }
        public Pubnub PNInstance = new Pubnub(_config);

        private void NotifyStateChanged()
        {
            OnChange?.Invoke();
        }
    }
}
