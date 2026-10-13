import React from "react";
import { useTranslation } from "react-i18next";
import { Slider } from "../ui/Slider";
import { useSettings } from "../../hooks/useSettings";

export const OtherAudioVolumeSlider: React.FC<{ disabled?: boolean }> = ({
  disabled = false,
}) => {
  const { t } = useTranslation();
  const { getSetting, updateSetting } = useSettings();
  const otherAudioVolume = getSetting("other_audio_volume") ?? 0.5;

  return (
    <Slider
      value={otherAudioVolume}
      onChange={(value: number) => updateSetting("other_audio_volume", value)}
      min={0}
      max={1}
      label={t("settings.sound.otherAudioVolume.title")}
      description={t("settings.sound.otherAudioVolume.description")}
      descriptionMode="tooltip"
      grouped
      formatValue={(value) => `${Math.round(value * 100)}%`}
      disabled={disabled}
    />
  );
};
