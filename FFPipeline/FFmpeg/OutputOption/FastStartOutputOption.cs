﻿namespace FFPipeline.FFmpeg.OutputOption;

public class FastStartOutputOption : OutputOption
{
    public override string[] OutputOptions => new[] { "-movflags", "+faststart" };
}
