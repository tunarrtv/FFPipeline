﻿namespace FFPipeline.FFmpeg.GlobalOption;

public class NoStatsOption : GlobalOption
{
    public override string[] GlobalOptions => new[] { "-nostats" };
}
