using NathanWiles.Nala.IO;
using System;
using System.Collections.Generic;
using System.Text;

namespace NathanWiles.Nala.Errors
{
    interface ErrorReporter
    {
        public void Report(IIOContext ioContext);
    }
}
