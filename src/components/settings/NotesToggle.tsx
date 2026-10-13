import React from "react";
import { useTranslation } from "react-i18next";
import { ToggleSwitch } from "../ui/ToggleSwitch";
import { useSettings } from "../../hooks/useSettings";

interface NotesToggleProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const NotesToggle: React.FC<NotesToggleProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const enabled = getSetting("notes_enabled") || false;

    return (
      <ToggleSwitch
        checked={enabled}
        onChange={(enabled) => updateSetting("notes_enabled", enabled)}
        isUpdating={isUpdating("notes_enabled")}
        label={t("settings.advanced.notesToggle.label")}
        description={t("settings.advanced.notesToggle.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      />
    );
  },
);
