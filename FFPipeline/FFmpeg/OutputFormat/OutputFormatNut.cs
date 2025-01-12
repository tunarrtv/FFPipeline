﻿using FFPipeline.FFmpeg.Environment;

namespace FFPipeline.FFmpeg.OutputFormat;

public class OutputFormatNut : IPipelineStep
{
    public EnvironmentVariable[] EnvironmentVariables => Array.Empty<EnvironmentVariable>();
    public string[] GlobalOptions => Array.Empty<string>();
    public string[] InputOptions(InputFile inputFile) => Array.Empty<string>();
    public string[] FilterOptions => Array.Empty<string>();

    public string[] OutputOptions => ["-f", "nut"];
    public FrameState NextState(FrameState currentState) => currentState;
}
