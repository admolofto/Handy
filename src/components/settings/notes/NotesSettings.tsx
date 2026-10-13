import React from "react";
import { useTranslation } from "react-i18next";
import { SettingsGroup } from "../../ui/SettingsGroup";
import { ShortcutInput } from "../ShortcutInput";
import { NotesDirectory } from "./NotesDirectory";

export const NotesSettings: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="max-w-3xl w-full mx-auto space-y-6">
      <SettingsGroup title={t("settings.notes.hotkey.title")}>
        <ShortcutInput
          shortcutId="transcribe_to_note"
          descriptionMode="tooltip"
          grouped={true}
        />
      </SettingsGroup>

      <SettingsGroup title={t("settings.notes.folder.groupTitle")}>
        <NotesDirectory descriptionMode="inline" grouped={true} />
      </SettingsGroup>
    </div>
  );
};
